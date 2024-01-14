import { primordials } from "ext:core/mod.js";

const { ObjectIs } = primordials;

export function sameValueZero(a, b) {
  return a === b || ObjectIs(a, b);
}
