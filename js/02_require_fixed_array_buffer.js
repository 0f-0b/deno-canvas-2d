import { primordials } from "ext:core/mod.js";
import { tryGetArrayBufferResizable } from "./01_try_get_array_buffer_resizable.js";

const { TypeError } = primordials;

export function requireFixedArrayBuffer(o) {
  const resizable = tryGetArrayBufferResizable(o);
  if (resizable === undefined) {
    throw new TypeError("Expected ArrayBuffer");
  }
  if (resizable) {
    throw new TypeError("ArrayBuffer must not be resizable");
  }
  return o;
}
