import { primordials } from "ext:core/mod.js";

const { ObjectDefineProperty, Promise } = primordials;

export function makeSafePromise(promise) {
  return ObjectDefineProperty(promise, "constructor", {
    __proto__: null,
    value: Promise,
  });
}
