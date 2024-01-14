import { primordials } from "ext:core/mod.js";

const { MathTrunc, NumberMAX_SAFE_INTEGER, TypeError } = primordials;
export const convertEnforceRangeUnsignedLongLong = (value) => {
  value = MathTrunc(value);
  if (!(value >= 0 && value <= NumberMAX_SAFE_INTEGER)) {
    throw new TypeError("Value is out of range");
  }
  return value || 0;
};
