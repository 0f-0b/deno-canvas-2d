import { ImageDataPrototypeGetData } from "ext:deno_canvas_2d/00_image_data_primordials.js";

export function tryGetImageDataData(o) {
  try {
    return ImageDataPrototypeGetData(o);
  } catch {
    return undefined;
  }
}
