export function isObject(o) {
  return typeof o === "object" ? o !== null : typeof o === "function";
}
