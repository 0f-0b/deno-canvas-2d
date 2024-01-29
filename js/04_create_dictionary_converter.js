import { requireObject } from "ext:canvas_2d/01_require_object.js";
import { primordials } from "ext:core/mod.js";

const { ObjectFreeze } = primordials;
const nullDict = ObjectFreeze({ __proto__: null });

export function createDictionaryConverter(readMembers) {
  return (value) => readMembers(requireObject(value ?? nullDict));
}
