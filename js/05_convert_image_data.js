import { isImageData } from "ext:canvas_2d/02_is_image_data.js";
import { primordials } from "ext:core/mod.js";

const { TypeError } = primordials;
export const convertImageData = (value) => {
  if (!isImageData(value)) {
    throw new TypeError("Expected ImageData");
  }
  return value;
};
