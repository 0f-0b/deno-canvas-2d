import { core, primordials } from "ext:core/mod.js";
import { isObject } from "./01_is_object.js";

const { ObjectGetOwnPropertyDescriptor, ObjectHasOwn } = primordials;
const { isProxy } = core;

export function capturePrototype(constructor, fallback) {
  if (constructor === fallback) {
    return constructor;
  }
  if (!isProxy(constructor)) {
    const desc = ObjectGetOwnPropertyDescriptor(constructor, "prototype");
    if (desc && ObjectHasOwn(desc, "value") && isObject(desc.value)) {
      return constructor;
    }
  }
  const proto = constructor.prototype;
  if (!isObject(proto)) {
    return fallback;
  }
  const C = function () {};
  C.prototype = proto;
  return C;
}
