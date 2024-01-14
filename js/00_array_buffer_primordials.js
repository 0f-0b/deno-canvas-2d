import { primordials } from "ext:core/mod.js";

const {
  ArrayBufferPrototype,
  ObjectGetOwnPropertyDescriptors,
  uncurryThis,
} = primordials;
const proto = ObjectGetOwnPropertyDescriptors(ArrayBufferPrototype);
export const ArrayBufferPrototypeGetResizable = uncurryThis(
  proto.resizable.get,
);
