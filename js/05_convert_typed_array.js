import { primordials } from "ext:core/mod.js";
import { requireFixedArrayBuffer } from "./02_require_fixed_array_buffer.js";

const { TypedArrayPrototypeGetBuffer } = primordials;
export const convertTypedArray = (value) => {
  requireFixedArrayBuffer(TypedArrayPrototypeGetBuffer(value));
  return value;
};
