import { primordials } from "ext:core/mod.js";

const { NumberIsFinite, TypeError } = primordials;
export const convertDouble = (value) => {
  value = +value;
  if (!NumberIsFinite(value)) {
    throw new TypeError("Value is out of range");
  }
  return value;
};
