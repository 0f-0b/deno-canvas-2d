import { primordials } from "ext:core/mod.js";

const { TypedArrayPrototypeGetSymbolToStringTag } = primordials;

export function isTypedArray(o) {
  return TypedArrayPrototypeGetSymbolToStringTag(o) !== undefined;
}
