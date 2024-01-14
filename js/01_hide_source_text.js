import { primordials } from "ext:core/mod.js";

const {
  FunctionPrototype,
  FunctionPrototypeToString,
  ObjectDefineProperty,
  SafeMap,
} = primordials;
const sourceTextMap = new SafeMap();

export function hideSourceText(fn, name) {
  if (name !== undefined) {
    ObjectDefineProperty(fn, "name", { __proto__: null, value: name });
  }
  sourceTextMap.set(fn, `function ${fn.name}() { [native code] }`);
  return fn;
}

ObjectDefineProperty(FunctionPrototype, "toString", {
  __proto__: null,
  value: hideSourceText(function toString() {
    return sourceTextMap.get(this) ?? FunctionPrototypeToString(this);
  }),
});
