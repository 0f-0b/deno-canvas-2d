import { primordials } from "ext:core/mod.js";
import { ImageDataPrototype } from "ext:deno_web/16_image_data.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
const proto = ObjectGetOwnPropertyDescriptors(ImageDataPrototype);
export const ImageDataPrototypeGetColorSpace = uncurryThis(
  proto.colorSpace.get,
);
export const ImageDataPrototypeGetData = uncurryThis(proto.data.get);
export const ImageDataPrototypeGetHeight = uncurryThis(proto.height.get);
export const ImageDataPrototypeGetWidth = uncurryThis(proto.width.get);
