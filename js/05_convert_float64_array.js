import { requireFixedArrayBuffer } from "ext:canvas_2d/02_require_fixed_array_buffer.js";
import { primordials } from "ext:core/mod.js";

const {
  TypeError,
  TypedArrayPrototypeGetBuffer,
  TypedArrayPrototypeGetSymbolToStringTag,
} = primordials;
export const convertFloat64Array = (value) => {
  const name = TypedArrayPrototypeGetSymbolToStringTag(value);
  if (name !== "Float64Array") {
    throw new TypeError(
      `Expected Float64Array, got ${name ?? "non-TypedArray"}`,
    );
  }
  requireFixedArrayBuffer(TypedArrayPrototypeGetBuffer(value));
  return value;
};
