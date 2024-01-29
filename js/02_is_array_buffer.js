import { tryGetArrayBufferResizable } from "ext:canvas_2d/01_try_get_array_buffer_resizable.js";

export function isArrayBuffer(o) {
  return tryGetArrayBufferResizable(o) !== undefined;
}
