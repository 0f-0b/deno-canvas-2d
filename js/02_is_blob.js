import { tryGetBlobSize } from "ext:canvas_2d/01_try_get_blob_size.js";

export function isBlob(o) {
  return tryGetBlobSize(o) !== undefined;
}
