use std::cell::Cell;
use std::ffi::c_void;
use std::rc::Rc;

use deno_core::{op2, v8, OpState};
use euclid::default::Transform2D;
use strum_macros::FromRepr;

use super::gc::{borrow_v8, from_v8, into_v8};
use super::image_bitmap::ImageBitmap;
use super::{raqote_ext, CanvasColorSpace};

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub(super) enum RepetitionBehavior {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
}

#[derive(Debug)]
pub(super) struct CanvasPattern {
    image: ImageBitmap,
    repetition_behavior: RepetitionBehavior,
    transformation_matrix: Cell<Transform2D<f64>>,
}

impl CanvasPattern {
    pub fn new(image: ImageBitmap, repetition: RepetitionBehavior) -> Self {
        Self {
            image,
            repetition_behavior: repetition,
            transformation_matrix: Cell::new(Transform2D::identity()),
        }
    }

    pub fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.transformation_matrix
            .set(Transform2D::new(a, b, c, d, e, f));
    }

    pub fn to_raqote(
        &self,
        destination_color_space: CanvasColorSpace,
        image_smoothing_enabled: bool,
    ) -> Option<raqote_ext::OwnedSource> {
        let image = self.image.clone().into_color_space(destination_color_space);
        let data = image.data?;
        let width = image.width.try_into().ok()?;
        let height = image.height.try_into().ok()?;
        let inverse_transform = self.transformation_matrix.get().inverse()?;
        Some(raqote_ext::OwnedSource::Image(
            raqote_ext::OwnedImage {
                width,
                height,
                data,
            },
            raqote::ExtendMode::Repeat,
            if image_smoothing_enabled {
                raqote::FilterMode::Bilinear
            } else {
                raqote::FilterMode::Nearest
            },
            inverse_transform.cast(),
            matches!(
                self.repetition_behavior,
                RepetitionBehavior::Repeat | RepetitionBehavior::RepeatX
            ),
            matches!(
                self.repetition_behavior,
                RepetitionBehavior::Repeat | RepetitionBehavior::RepeatY
            ),
        ))
    }
}

#[op2]
pub fn op_canvas_2d_pattern_new<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    image: *const c_void,
    repetition: i32,
) -> v8::Local<'a, v8::External> {
    let image = from_v8::<ImageBitmap>(state, image);
    let repetition = RepetitionBehavior::from_repr(repetition).unwrap();
    let result = Rc::new(CanvasPattern::new(image, repetition));
    into_v8(state, scope, result)
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_pattern_set_transform(
    state: &OpState,
    this: *const c_void,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    let this = borrow_v8::<Rc<CanvasPattern>>(state, this);
    if [a, b, c, d, e, f].into_iter().all(f64::is_finite) {
        this.set_transform(a, b, c, d, e, f);
    }
}
