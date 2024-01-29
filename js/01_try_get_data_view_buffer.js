import { primordials } from "ext:core/mod.js";

const { DataViewPrototypeGetBuffer } = primordials;

export function tryGetDataViewBuffer(o) {
  try {
    return DataViewPrototypeGetBuffer(o);
  } catch {
    return undefined;
  }
}
