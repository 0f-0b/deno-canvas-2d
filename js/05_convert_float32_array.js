import { primordials } from "ext:core/mod.js";
import { requireFixedArrayBuffer } from "./02_require_fixed_array_buffer.js";

const {
  TypeError,
  TypedArrayPrototypeGetBuffer,
  TypedArrayPrototypeGetSymbolToStringTag,
} = primordials;
export const convertFloat32Array = (value) => {
  const name = TypedArrayPrototypeGetSymbolToStringTag(value);
  if (name !== "Float32Array") {
    throw new TypeError(
      `Expected Float32Array, got ${name ?? "non-TypedArray"}`,
    );
  }
  requireFixedArrayBuffer(TypedArrayPrototypeGetBuffer(value));
  return value;
};
