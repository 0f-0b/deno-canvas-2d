import { primordials } from "ext:core/mod.js";

const { MathTrunc, TypeError } = primordials;
export const convertEnforceRangeUnsignedLong = (value) => {
  value = MathTrunc(value);
  if (!(value >= 0 && value <= 0xffffffff)) {
    throw new TypeError("Value is out of range");
  }
  return value || 0;
};
