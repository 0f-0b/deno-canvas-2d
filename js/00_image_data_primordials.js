import { primordials } from "ext:core/mod.js";
import { ImageData } from "ext:deno_web/16_image_data.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
export const ImageDataPrototype = ImageData.prototype;
const proto = ObjectGetOwnPropertyDescriptors(ImageDataPrototype);
export const ImageDataPrototypeGetWidth = uncurryThis(proto.width.get);
export const ImageDataPrototypeGetHeight = uncurryThis(proto.height.get);
export const ImageDataPrototypeGetData = uncurryThis(proto.data.get);
export const ImageDataPrototypeGetColorSpace = uncurryThis(
  proto.colorSpace.get,
);
