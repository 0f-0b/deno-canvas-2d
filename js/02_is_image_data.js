import { tryGetImageDataData } from "./01_try_get_image_data_data.js";

export function isImageData(o) {
  return tryGetImageDataData(o) !== undefined;
}
