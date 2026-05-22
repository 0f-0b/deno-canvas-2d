import { core, primordials } from "ext:core/mod.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
const { loadExtScript } = core;
const { Blob } = loadExtScript("ext:deno_web/09_file.js");
export const BlobPrototype = Blob.prototype;
const proto = ObjectGetOwnPropertyDescriptors(BlobPrototype);
export const BlobPrototypeBytes = uncurryThis(proto.bytes.value);
export const BlobPrototypeGetSize = uncurryThis(proto.size.get);
export const BlobPrototypeGetType = uncurryThis(proto.type.get);
