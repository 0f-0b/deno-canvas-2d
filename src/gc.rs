use std::any::TypeId;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::collections::HashSet;
use std::ffi::c_void;
use std::rc::Rc;

use deno_core::{v8, OpState};

fn into_managed<'a, T: 'static>(
    scope: &mut v8::HandleScope<'a>,
    handle: impl v8::Handle<Data = T>,
    weak_rc: Rc<Cell<Option<v8::Weak<T>>>>,
    finalizer: impl FnOnce() + 'static,
) -> v8::Local<'a, T> {
    let weak = v8::Weak::with_finalizer(
        scope,
        handle,
        Box::new({
            let weak_rc = weak_rc.clone();
            move |_| {
                weak_rc.take();
                finalizer();
            }
        }),
    );
    let local = weak.to_local(scope).unwrap();
    weak_rc.set(Some(weak));
    local
}

struct ExternalBox<T> {
    weak_rc: Rc<Cell<Option<v8::Weak<v8::External>>>>,
    value: RefCell<T>,
}

#[derive(Clone, Debug, Default)]
pub struct ExternalRegistry {
    inner: Rc<RefCell<HashSet<(TypeId, *mut c_void)>>>,
}

impl ExternalRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert<'a, T: 'static>(
        &self,
        scope: &mut v8::HandleScope<'a>,
        value: T,
    ) -> v8::Local<'a, v8::External> {
        let inner_rc = self.inner.clone();
        let weak_rc = Rc::new(Cell::new(None));
        let ptr = Box::into_raw(Box::new(ExternalBox {
            weak_rc: weak_rc.clone(),
            value: RefCell::new(value),
        }));
        let key = (TypeId::of::<T>(), ptr.cast());
        assert!(inner_rc.borrow_mut().insert(key));
        let handle = v8::External::new(scope, ptr.cast());
        into_managed(scope, handle, weak_rc, move || {
            assert!(inner_rc.borrow_mut().remove(&key));
            drop(unsafe { Box::from_raw(ptr) });
        })
    }

    pub fn get<T: 'static>(&self, ptr: *const c_void) -> Option<Ref<'_, T>> {
        let inner = self.inner.borrow();
        let key = (TypeId::of::<T>(), ptr.cast_mut());
        inner.contains(&key).then(|| {
            let ptr = ptr.cast::<ExternalBox<T>>();
            unsafe { &(*ptr).value }.borrow()
        })
    }

    pub fn get_mut<T: 'static>(&self, ptr: *const c_void) -> Option<RefMut<'_, T>> {
        let inner = self.inner.borrow();
        let key = (TypeId::of::<T>(), ptr.cast_mut());
        inner.contains(&key).then(|| {
            let ptr = ptr.cast::<ExternalBox<T>>();
            unsafe { &(*ptr).value }.borrow_mut()
        })
    }

    pub fn take<T: 'static>(&self, ptr: *const c_void) -> Option<T> {
        let mut inner = self.inner.borrow_mut();
        let key = (TypeId::of::<T>(), ptr.cast_mut());
        inner.remove(&key).then(|| {
            let ptr = ptr.cast::<ExternalBox<T>>();
            unsafe { &(*ptr).value }.borrow_mut();
            let inner = unsafe { Box::from_raw(ptr.cast_mut()) };
            inner.weak_rc.take();
            inner.value.into_inner()
        })
    }
}

pub fn into_v8<'a, T: 'static>(
    state: &OpState,
    scope: &mut v8::HandleScope<'a>,
    value: T,
) -> v8::Local<'a, v8::External> {
    let registry = state.borrow::<ExternalRegistry>();
    registry.insert(scope, value)
}

pub fn from_v8<T: 'static>(state: &OpState, ptr: *const c_void) -> T {
    let registry = state.borrow::<ExternalRegistry>();
    registry.take::<T>(ptr).unwrap()
}

pub fn borrow_v8<T: 'static>(state: &OpState, ptr: *const c_void) -> Ref<'_, T> {
    let registry = state.borrow::<ExternalRegistry>();
    registry.get::<T>(ptr).unwrap()
}

pub fn borrow_v8_mut<T: 'static>(state: &OpState, ptr: *const c_void) -> RefMut<'_, T> {
    let registry = state.borrow::<ExternalRegistry>();
    registry.get_mut::<T>(ptr).unwrap()
}

pub fn init(state: &mut OpState) {
    state.put(ExternalRegistry::new());
}
