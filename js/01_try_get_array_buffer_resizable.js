import { primordials } from "ext:core/mod.js";

const { ArrayBufferPrototypeGetResizable } = primordials;

export function tryGetArrayBufferResizable(o) {
  try {
    return ArrayBufferPrototypeGetResizable(o);
  } catch {
    return undefined;
  }
}
