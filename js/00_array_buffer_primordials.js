import { primordials } from "ext:core/mod.js";

const {
  ArrayBufferPrototype,
  ObjectGetOwnPropertyDescriptors,
  uncurryThis,
} = primordials;
export let ArrayBufferPrototypeGetResizable;

export function initArrayBufferPrimordials() {
  const proto = ObjectGetOwnPropertyDescriptors(ArrayBufferPrototype);
  ArrayBufferPrototypeGetResizable = uncurryThis(proto.resizable.get);
}
