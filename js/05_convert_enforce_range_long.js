import { primordials } from "ext:core/mod.js";

const { MathTrunc, TypeError } = primordials;
export const convertEnforceRangeLong = (value) => {
  value = MathTrunc(value);
  if (!(value >= -0x80000000 && value <= 0x7fffffff)) {
    throw new TypeError("Value is out of range");
  }
  return value || 0;
};
