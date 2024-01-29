import { requireFixedArrayBuffer } from "ext:canvas_2d/02_require_fixed_array_buffer.js";
import { primordials } from "ext:core/mod.js";

const { DataViewPrototypeGetBuffer } = primordials;
export const convertDataView = (value) => {
  requireFixedArrayBuffer(DataViewPrototypeGetBuffer(value));
  return value;
};
