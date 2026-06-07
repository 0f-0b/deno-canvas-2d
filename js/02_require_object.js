import { primordials } from "ext:core/mod.js";
import { isObject } from "./01_is_object.js";

const { String, TypeError } = primordials;

export function requireObject(value) {
  if (!isObject(value)) {
    throw new TypeError(`${String(value)} is not an object`);
  }
  return value;
}
