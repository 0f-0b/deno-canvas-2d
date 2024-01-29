import { BlobPrototypeGetSize } from "ext:canvas_2d/00_blob_primordials.js";

export function tryGetBlobSize(o) {
  try {
    return BlobPrototypeGetSize(o);
  } catch {
    return undefined;
  }
}
