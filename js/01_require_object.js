import { primordials } from "ext:core/mod.js";
import { type } from "ext:deno_webidl/00_webidl.js";

const { String, TypeError } = primordials;

export function requireObject(value) {
  if (type(value) !== "Object") {
    throw new TypeError(`${String(value)} is not an object`);
  }
  return value;
}
