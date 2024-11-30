import { primordials } from "ext:core/mod.js";

const {
  ArrayPrototypePush,
  ObjectDefineProperty,
  ObjectSetPrototypeOf,
  Promise,
  PromisePrototypeThen,
  SafeArrayIterator,
  makeSafe,
} = primordials;
const SafePromise = makeSafe(Promise, class extends Promise {});

export function makeSafePromise(promise) {
  return ObjectDefineProperty(promise, "constructor", {
    __proto__: null,
    value: Promise,
  });
}

export function makeSpeciesSafePromise(promise) {
  return ObjectDefineProperty(promise, "constructor", {
    __proto__: null,
    value: undefined,
  });
}

export function newFromSpeciesSafePromise(promise) {
  return new Promise((resolve, reject) =>
    PromisePrototypeThen(promise, resolve, reject)
  );
}

export function aggregateSpeciesSafePromises(promises) {
  const safePromises = ObjectSetPrototypeOf([], null);
  for (const promise of new SafeArrayIterator(promises)) {
    ArrayPrototypePush(
      safePromises,
      new SafePromise((resolve, reject) =>
        PromisePrototypeThen(promise, resolve, reject)
      ),
    );
  }
  return newFromSpeciesSafePromise(
    SafePromise.all(new SafeArrayIterator(safePromises)),
  );
}
