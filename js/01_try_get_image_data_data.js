import { ImageDataPrototypeGetData } from "./00_image_data_primordials.js";

export function tryGetImageDataData(o) {
  try {
    return ImageDataPrototypeGetData(o);
  } catch {
    return undefined;
  }
}
