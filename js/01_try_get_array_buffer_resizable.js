import { ArrayBufferPrototypeGetResizable } from "ext:canvas_2d/00_array_buffer_primordials.js";

export function tryGetArrayBufferResizable(o) {
  try {
    return ArrayBufferPrototypeGetResizable(o);
  } catch {
    return undefined;
  }
}
