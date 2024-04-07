import { primordials } from "ext:core/mod.js";
import { requireFixedArrayBuffer } from "./02_require_fixed_array_buffer.js";

const { DataViewPrototypeGetBuffer } = primordials;
export const convertDataView = (value) => {
  requireFixedArrayBuffer(DataViewPrototypeGetBuffer(value));
  return value;
};
