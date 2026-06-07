import { primordials } from "ext:core/mod.js";
import { requireObject } from "./02_require_object.js";

const { ObjectFreeze } = primordials;
const nullDict = ObjectFreeze({ __proto__: null });

export function createDictionaryConverter(readMembers) {
  return (value) => readMembers(requireObject(value ?? nullDict));
}
