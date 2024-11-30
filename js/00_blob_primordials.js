import { primordials } from "ext:core/mod.js";
import { Blob } from "ext:deno_web/09_file.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
export const BlobPrototype = Blob.prototype;
const proto = ObjectGetOwnPropertyDescriptors(BlobPrototype);
export const BlobPrototypeBytes = uncurryThis(proto.bytes.value);
export const BlobPrototypeGetSize = uncurryThis(proto.size.get);
export const BlobPrototypeGetType = uncurryThis(proto.type.get);
