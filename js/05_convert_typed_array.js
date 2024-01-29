import { requireFixedArrayBuffer } from "ext:canvas_2d/02_require_fixed_array_buffer.js";
import { primordials } from "ext:core/mod.js";

const { TypedArrayPrototypeGetBuffer } = primordials;
export const convertTypedArray = (value) => {
  requireFixedArrayBuffer(TypedArrayPrototypeGetBuffer(value));
  return value;
};
