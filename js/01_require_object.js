import { core, primordials } from "ext:core/mod.js";

const { String, TypeError } = primordials;
const { loadExtScript } = core;
const { type } = loadExtScript("ext:deno_webidl/00_webidl.js");

export function requireObject(value) {
  if (type(value) !== "Object") {
    throw new TypeError(`${String(value)} is not an object`);
  }
  return value;
}
