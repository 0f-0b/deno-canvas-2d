import { core, primordials } from "ext:core/mod.js";
import { type } from "ext:deno_webidl/00_webidl.js";

const { ObjectGetOwnPropertyDescriptor, ObjectHasOwn } = primordials;
const { isProxy } = core;

export function capturePrototype(constructor, fallback) {
  if (constructor === fallback) {
    return constructor;
  }
  if (!isProxy(constructor)) {
    const desc = ObjectGetOwnPropertyDescriptor(constructor, "prototype");
    if (desc && ObjectHasOwn(desc, "value") && type(desc.value) === "Object") {
      return constructor;
    }
  }
  const proto = constructor.prototype;
  if (type(proto) !== "Object") {
    return fallback;
  }
  const C = function () {};
  C.prototype = proto;
  return C;
}
