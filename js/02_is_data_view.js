import { tryGetDataViewBuffer } from "ext:canvas_2d/01_try_get_data_view_buffer.js";

export function isDataView(o) {
  return tryGetDataViewBuffer(o) !== undefined;
}
