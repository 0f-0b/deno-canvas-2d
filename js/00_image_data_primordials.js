import { primordials } from "ext:core/mod.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
export let ImageData;
export let ImageDataPrototype;
export let ImageDataPrototypeGetColorSpace;
export let ImageDataPrototypeGetData;
export let ImageDataPrototypeGetHeight;
export let ImageDataPrototypeGetWidth;

export function setImageData(constructor) {
  ImageData = constructor;
  ImageDataPrototype = ImageData.prototype;
  const proto = ObjectGetOwnPropertyDescriptors(ImageDataPrototype);
  ImageDataPrototypeGetColorSpace = uncurryThis(proto.colorSpace.get);
  ImageDataPrototypeGetData = uncurryThis(proto.data.get);
  ImageDataPrototypeGetHeight = uncurryThis(proto.height.get);
  ImageDataPrototypeGetWidth = uncurryThis(proto.width.get);
}
