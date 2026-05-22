import { primordials } from "ext:core/mod.js";
import { sameValueZero } from "./01_same_value_zero.js";
import { createDictionaryConverter } from "./04_create_dictionary_converter.js";
import { convertUnrestrictedDouble } from "./05_convert_unrestricted_double.js";

const { TypeError } = primordials;
const readDOMPointInitMembers = (value) => {
  const result = { __proto__: null };
  const { w = 1 } = value;
  result.w = convertUnrestrictedDouble(w);
  const { x = 0 } = value;
  result.x = convertUnrestrictedDouble(x);
  const { y = 0 } = value;
  result.y = convertUnrestrictedDouble(y);
  const { z = 0 } = value;
  result.z = convertUnrestrictedDouble(z);
  return result;
};
export const convertDOMPointInit = createDictionaryConverter(
  readDOMPointInitMembers,
);
const readDOMMatrix2DInitMembers = (value) => {
  const result = { __proto__: null };
  const { a } = value;
  if (a !== undefined) {
    result.a = convertUnrestrictedDouble(a);
  }
  const { b } = value;
  if (b !== undefined) {
    result.b = convertUnrestrictedDouble(b);
  }
  const { c } = value;
  if (c !== undefined) {
    result.c = convertUnrestrictedDouble(c);
  }
  const { d } = value;
  if (d !== undefined) {
    result.d = convertUnrestrictedDouble(d);
  }
  const { e } = value;
  if (e !== undefined) {
    result.e = convertUnrestrictedDouble(e);
  }
  const { f } = value;
  if (f !== undefined) {
    result.f = convertUnrestrictedDouble(f);
  }
  const { m11 } = value;
  if (m11 !== undefined) {
    result.m11 = convertUnrestrictedDouble(m11);
  }
  const { m12 } = value;
  if (m12 !== undefined) {
    result.m12 = convertUnrestrictedDouble(m12);
  }
  const { m21 } = value;
  if (m21 !== undefined) {
    result.m21 = convertUnrestrictedDouble(m21);
  }
  const { m22 } = value;
  if (m22 !== undefined) {
    result.m22 = convertUnrestrictedDouble(m22);
  }
  const { m41 } = value;
  if (m41 !== undefined) {
    result.m41 = convertUnrestrictedDouble(m41);
  }
  const { m42 } = value;
  if (m42 !== undefined) {
    result.m42 = convertUnrestrictedDouble(m42);
  }
  return result;
};
export const convertDOMMatrix2DInit = createDictionaryConverter(
  readDOMMatrix2DInitMembers,
);

export function validateAndFixup2D(other) {
  const { a, b, c, d, e, f, m11, m12, m21, m22, m41, m42 } = other;
  if (
    !((a === undefined || m11 === undefined || sameValueZero(a, m11)) &&
      (b === undefined || m12 === undefined || sameValueZero(b, m12)) &&
      (c === undefined || m21 === undefined || sameValueZero(c, m21)) &&
      (d === undefined || m22 === undefined || sameValueZero(d, m22)) &&
      (e === undefined || m41 === undefined || sameValueZero(e, m41)) &&
      (f === undefined || m42 === undefined || sameValueZero(f, m42)))
  ) {
    throw new TypeError("Invalid matrix");
  }
  other.m11 = m11 ?? a ?? 1;
  other.m12 = m12 ?? b ?? 0;
  other.m21 = m21 ?? c ?? 0;
  other.m22 = m22 ?? d ?? 1;
  other.m41 = m41 ?? e ?? 0;
  other.m42 = m42 ?? f ?? 0;
}
