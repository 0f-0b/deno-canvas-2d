import { primordials } from "ext:core/mod.js";
import { isImageData } from "ext:deno_canvas_2d/02_is_image_data.js";

const { TypeError } = primordials;
export const convertImageData = (value) => {
  if (!isImageData(value)) {
    throw new TypeError("Expected ImageData");
  }
  return value;
};
