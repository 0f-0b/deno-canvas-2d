import { primordials } from "ext:core/mod.js";

const { SafeSet, TypeError } = primordials;

export function createEnumConverter(name, values) {
  const set = new SafeSet(values);
  return (value) => {
    value = `${value}`;
    if (!set.has(value)) {
      throw new TypeError(`Invalid value '${value}' for enum ${name}`);
    }
    return value;
  };
}
