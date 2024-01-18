import { primordials } from "ext:core/mod.js";
import { sameValueZero } from "ext:deno_canvas_2d/01_same_value_zero.js";
import { createDictionaryConverter } from "ext:deno_canvas_2d/04_create_dictionary_converter.js";
import { createSequenceFromIterable } from "ext:deno_canvas_2d/04_create_sequence_from_iterable.js";
import { convertBoolean } from "ext:deno_canvas_2d/05_convert_boolean.js";
import { convertDOMString } from "ext:deno_canvas_2d/05_convert_dom_string.js";
import { convertFloat32Array } from "ext:deno_canvas_2d/05_convert_float32_array.js";
import { convertFloat64Array } from "ext:deno_canvas_2d/05_convert_float64_array.js";
import { convertUnrestrictedDouble } from "ext:deno_canvas_2d/05_convert_unrestricted_double.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import {
  configureInterface,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";

const {
  Float32Array,
  Float64Array,
  MathAtan2,
  MathCos,
  MathHypot,
  MathMax,
  MathMin,
  MathSin,
  MathTan,
  ObjectFreeze,
  Symbol,
  SymbolFor,
  SymbolIterator,
  TypeError,
  TypedArrayPrototypeGetLength,
} = primordials;
const privateCustomInspect = SymbolFor("Deno.privateCustomInspect");
const RAD_PER_DEG = 0.017453292519943295;

function radians(degrees) {
  return degrees * RAD_PER_DEG;
}

let getDOMPointX;
let setDOMPointX;
let getDOMPointY;
let setDOMPointY;
let getDOMPointZ;
let setDOMPointZ;
let getDOMPointW;
let setDOMPointW;

export class DOMPointReadOnly {
  #brand() {}

  #x;
  #y;
  #z;
  #w;

  constructor(x = 0, y = 0, z = 0, w = 1) {
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    z = convertUnrestrictedDouble(z);
    w = convertUnrestrictedDouble(w);
    this.#x = x;
    this.#y = y;
    this.#z = z;
    this.#w = w;
  }

  static fromPoint(other = undefined) {
    other = convertDOMPointInit(other);
    return createDOMPointReadOnlyFromDictionary(other);
  }

  get x() {
    return this.#x;
  }

  get y() {
    return this.#y;
  }

  get z() {
    return this.#z;
  }

  get w() {
    return this.#w;
  }

  matrixTransform(matrix = undefined) {
    this.#brand;
    matrix = convertDOMMatrixInit(matrix);
    const matrixObject = createDOMMatrixReadOnlyFromDictionary(matrix);
    return transformPointWithMatrix(this, matrixObject);
  }

  toJSON() {
    this.#brand;
    return { x: this.#x, y: this.#y, z: this.#z, w: this.#w };
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["x", "y", "z", "w"],
      }),
      options,
    );
  }

  get [privateCustomInspect]() {
    try {
      return this.#inspect;
    } catch {
      return undefined;
    }
  }

  static {
    configureInterface(this);
    getDOMPointX = (o) => o.#x;
    setDOMPointX = (o, v) => o.#x = v;
    getDOMPointY = (o) => o.#y;
    setDOMPointY = (o, v) => o.#y = v;
    getDOMPointZ = (o) => o.#z;
    setDOMPointZ = (o, v) => o.#z = v;
    getDOMPointW = (o) => o.#w;
    setDOMPointW = (o, v) => o.#w = v;
  }
}

export class DOMPoint extends DOMPointReadOnly {
  #brand() {}

  static fromPoint(other = undefined) {
    other = convertDOMPointInit(other);
    return createDOMPointFromDictionary(other);
  }

  get x() {
    this.#brand;
    return getDOMPointX(this);
  }

  set x(value) {
    this.#brand;
    const prefix = "Failed to set 'x' on 'DOMPoint'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMPointX(this, value);
  }

  get y() {
    this.#brand;
    return getDOMPointY(this);
  }

  set y(value) {
    this.#brand;
    const prefix = "Failed to set 'y' on 'DOMPoint'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMPointY(this, value);
  }

  get z() {
    this.#brand;
    return getDOMPointZ(this);
  }

  set z(value) {
    this.#brand;
    const prefix = "Failed to set 'z' on 'DOMPoint'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMPointZ(this, value);
  }

  get w() {
    this.#brand;
    return getDOMPointW(this);
  }

  set w(value) {
    this.#brand;
    const prefix = "Failed to set 'w' on 'DOMPoint'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMPointW(this, value);
  }

  static {
    configureInterface(this);
  }
}

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

function createDOMPointReadOnlyFromDictionary(other) {
  return new DOMPointReadOnly(other.x, other.y, other.z, other.w);
}

function createDOMPointFromDictionary(other) {
  return new DOMPoint(other.x, other.y, other.z, other.w);
}

function transformPointWithMatrix(point, matrix) {
  const x = getDOMPointX(point);
  const y = getDOMPointY(point);
  const z = getDOMPointZ(point);
  const w = getDOMPointW(point);
  const m11 = getDOMMatrixM11(matrix);
  const m12 = getDOMMatrixM12(matrix);
  const m13 = getDOMMatrixM13(matrix);
  const m14 = getDOMMatrixM14(matrix);
  const m21 = getDOMMatrixM21(matrix);
  const m22 = getDOMMatrixM22(matrix);
  const m23 = getDOMMatrixM23(matrix);
  const m24 = getDOMMatrixM24(matrix);
  const m31 = getDOMMatrixM31(matrix);
  const m32 = getDOMMatrixM32(matrix);
  const m33 = getDOMMatrixM33(matrix);
  const m34 = getDOMMatrixM34(matrix);
  const m41 = getDOMMatrixM41(matrix);
  const m42 = getDOMMatrixM42(matrix);
  const m43 = getDOMMatrixM43(matrix);
  const m44 = getDOMMatrixM44(matrix);
  return new DOMPoint(
    m11 * x + m21 * y + m31 * z + m41 * w,
    m12 * x + m22 * y + m32 * z + m42 * w,
    m13 * x + m23 * y + m33 * z + m43 * w,
    m14 * x + m24 * y + m34 * z + m44 * w,
  );
}

let getDOMRectX;
let setDOMRectX;
let getDOMRectY;
let setDOMRectY;
let getDOMRectWidth;
let setDOMRectWidth;
let getDOMRectHeight;
let setDOMRectHeight;

export class DOMRectReadOnly {
  #brand() {}

  #x;
  #y;
  #width;
  #height;

  constructor(x = 0, y = 0, width = 0, height = 0) {
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    width = convertUnrestrictedDouble(width);
    height = convertUnrestrictedDouble(height);
    this.#x = x;
    this.#y = y;
    this.#width = width;
    this.#height = height;
  }

  static fromRect(other = undefined) {
    other = convertDOMRectInit(other);
    return createDOMRectReadOnlyFromDictionary(other);
  }

  get x() {
    return this.#x;
  }

  get y() {
    return this.#y;
  }

  get width() {
    return this.#width;
  }

  get height() {
    return this.#height;
  }

  get #top() {
    return MathMin(this.#y, this.#y + this.#height);
  }

  get top() {
    return this.#top;
  }

  get #right() {
    return MathMax(this.#x, this.#x + this.#width);
  }

  get right() {
    return this.#right;
  }

  get #bottom() {
    return MathMax(this.#y, this.#y + this.#height);
  }

  get bottom() {
    return this.#bottom;
  }

  get #left() {
    return MathMin(this.#x, this.#x + this.#width);
  }

  get left() {
    return this.#left;
  }

  toJSON() {
    this.#brand;
    return {
      x: this.#x,
      y: this.#y,
      width: this.#width,
      height: this.#height,
      top: this.#top,
      right: this.#right,
      bottom: this.#bottom,
      left: this.#left,
    };
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["x", "y", "width", "height", "top", "right", "bottom", "left"],
      }),
      options,
    );
  }

  get [privateCustomInspect]() {
    try {
      return this.#inspect;
    } catch {
      return undefined;
    }
  }

  static {
    configureInterface(this);
    getDOMRectX = (o) => o.#x;
    setDOMRectX = (o, v) => o.#x = v;
    getDOMRectY = (o) => o.#y;
    setDOMRectY = (o, v) => o.#y = v;
    getDOMRectWidth = (o) => o.#width;
    setDOMRectWidth = (o, v) => o.#width = v;
    getDOMRectHeight = (o) => o.#height;
    setDOMRectHeight = (o, v) => o.#height = v;
  }
}

export class DOMRect extends DOMRectReadOnly {
  #brand() {}

  static fromRect(other = undefined) {
    other = convertDOMRectInit(other);
    return createDOMRectFromDictionary(other);
  }

  get x() {
    this.#brand;
    return getDOMRectX(this);
  }

  set x(value) {
    this.#brand;
    const prefix = "Failed to set 'x' on 'DOMRect'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMRectX(this, value);
  }

  get y() {
    this.#brand;
    return getDOMRectY(this);
  }

  set y(value) {
    this.#brand;
    const prefix = "Failed to set 'y' on 'DOMRect'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMRectY(this, value);
  }

  get width() {
    this.#brand;
    return getDOMRectWidth(this);
  }

  set width(value) {
    this.#brand;
    const prefix = "Failed to set 'width' on 'DOMRect'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMRectWidth(this, value);
  }

  get height() {
    this.#brand;
    return getDOMRectHeight(this);
  }

  set height(value) {
    this.#brand;
    const prefix = "Failed to set 'height' on 'DOMRect'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMRectHeight(this, value);
  }

  static {
    configureInterface(this);
  }
}

const readDOMRectInitMembers = (value) => {
  const result = { __proto__: null };
  const { height = 0 } = value;
  result.height = convertUnrestrictedDouble(height);
  const { width = 0 } = value;
  result.width = convertUnrestrictedDouble(width);
  const { x = 0 } = value;
  result.x = convertUnrestrictedDouble(x);
  const { y = 0 } = value;
  result.y = convertUnrestrictedDouble(y);
  return result;
};
const convertDOMRectInit = createDictionaryConverter(readDOMRectInitMembers);

function createDOMRectReadOnlyFromDictionary(other) {
  return new DOMRectReadOnly(other.x, other.y, other.width, other.height);
}

function createDOMRectFromDictionary(other) {
  return new DOMRect(other.x, other.y, other.width, other.height);
}

export class DOMQuad {
  #brand() {}

  #p1;
  #p2;
  #p3;
  #p4;

  constructor(p1 = undefined, p2, p3, p4) {
    p1 = convertDOMPointInit(p1);
    p2 = convertDOMPointInit(p2);
    p3 = convertDOMPointInit(p3);
    p4 = convertDOMPointInit(p4);
    this.#p1 = createDOMPointFromDictionary(p1);
    this.#p2 = createDOMPointFromDictionary(p2);
    this.#p3 = createDOMPointFromDictionary(p3);
    this.#p4 = createDOMPointFromDictionary(p4);
  }

  static fromRect(other = undefined) {
    other = convertDOMRectInit(other);
    return createDOMQuadFromDOMRectInitDictionary(other);
  }

  static fromQuad(other = undefined) {
    other = convertDOMQuadInit(other);
    return createDOMQuadFromDOMQuadInitDictionary(other);
  }

  get p1() {
    return this.#p1;
  }

  get p2() {
    return this.#p2;
  }

  get p3() {
    return this.#p3;
  }

  get p4() {
    return this.#p4;
  }

  getBounds() {
    this.#brand;
    const p1 = this.#p1;
    const p2 = this.#p2;
    const p3 = this.#p3;
    const p4 = this.#p4;
    const p1x = getDOMPointX(p1);
    const p2x = getDOMPointX(p2);
    const p3x = getDOMPointX(p3);
    const p4x = getDOMPointX(p4);
    const p1y = getDOMPointY(p1);
    const p2y = getDOMPointY(p2);
    const p3y = getDOMPointY(p3);
    const p4y = getDOMPointY(p4);
    const left = MathMin(p1x, p2x, p3x, p4x);
    const top = MathMin(p1y, p2y, p3y, p4y);
    const right = MathMax(p1x, p2x, p3x, p4x);
    const bottom = MathMax(p1y, p2y, p3y, p4y);
    return new DOMRect(left, top, right - left, bottom - top);
  }

  toJSON() {
    this.#brand;
    return { p1: this.#p1, p2: this.#p2, p3: this.#p3, p4: this.#p4 };
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["p1", "p2", "p3", "p4"],
      }),
      options,
    );
  }

  get [privateCustomInspect]() {
    try {
      return this.#inspect;
    } catch {
      return undefined;
    }
  }

  static {
    configureInterface(this);
  }
}

const readDOMQuadInitMembers = (value) => {
  const result = { __proto__: null };
  const { p1 } = value;
  if (p1 !== undefined) {
    result.p1 = convertDOMPointInit(p1);
  }
  const { p2 } = value;
  if (p2 !== undefined) {
    result.p2 = convertDOMPointInit(p2);
  }
  const { p3 } = value;
  if (p3 !== undefined) {
    result.p3 = convertDOMPointInit(p3);
  }
  const { p4 } = value;
  if (p4 !== undefined) {
    result.p4 = convertDOMPointInit(p4);
  }
  return result;
};
const convertDOMQuadInit = createDictionaryConverter(readDOMQuadInitMembers);

function createDOMQuadFromDOMRectInitDictionary(other) {
  return new DOMQuad(
    new DOMPoint(other.x, other.y),
    new DOMPoint(other.x + other.width, other.y),
    new DOMPoint(other.x + other.width, other.y + other.height),
    new DOMPoint(other.x, other.y + other.height),
  );
}

function createDOMQuadFromDOMQuadInitDictionary(other) {
  return new DOMQuad(
    other.p1 === undefined ? undefined : createDOMPointFromDictionary(other.p1),
    other.p2 === undefined ? undefined : createDOMPointFromDictionary(other.p2),
    other.p3 === undefined ? undefined : createDOMPointFromDictionary(other.p3),
    other.p4 === undefined ? undefined : createDOMPointFromDictionary(other.p4),
  );
}

const convertDOMStringOrSequenceOfUnrestrictedDouble = (value) => {
  if (type(value) === "Object") {
    const method = value[SymbolIterator];
    if (method !== null && method !== undefined) {
      return createSequenceFromIterable(
        value,
        method,
        convertUnrestrictedDouble,
      );
    }
  }
  return convertDOMString(value);
};
let getDOMMatrixM11;
let setDOMMatrixM11;
let getDOMMatrixM12;
let setDOMMatrixM12;
let getDOMMatrixM13;
let setDOMMatrixM13;
let getDOMMatrixM14;
let setDOMMatrixM14;
let getDOMMatrixM21;
let setDOMMatrixM21;
let getDOMMatrixM22;
let setDOMMatrixM22;
let getDOMMatrixM23;
let setDOMMatrixM23;
let getDOMMatrixM24;
let setDOMMatrixM24;
let getDOMMatrixM31;
let setDOMMatrixM31;
let getDOMMatrixM32;
let setDOMMatrixM32;
let getDOMMatrixM33;
let setDOMMatrixM33;
let getDOMMatrixM34;
let setDOMMatrixM34;
let getDOMMatrixM41;
let setDOMMatrixM41;
let getDOMMatrixM42;
let setDOMMatrixM42;
let getDOMMatrixM43;
let setDOMMatrixM43;
let getDOMMatrixM44;
let setDOMMatrixM44;
let getDOMMatrixIs2D;
let setDOMMatrixIs2D;
export const directConstruct = Symbol();
const identityMatrix2DValues = ObjectFreeze([1, 0, 0, 1, 0, 0]);

export class DOMMatrixReadOnly {
  #brand() {}

  #m11;
  #m12;
  #m13;
  #m14;
  #m21;
  #m22;
  #m23;
  #m24;
  #m31;
  #m32;
  #m33;
  #m34;
  #m41;
  #m42;
  #m43;
  #m44;
  #is2D;

  constructor(
    initOrKey = undefined,
    values = identityMatrix2DValues,
    is2D = true,
  ) {
    if (initOrKey === undefined) {
      values = identityMatrix2DValues;
      is2D = true;
    } else if (initOrKey !== directConstruct) {
      const init = convertDOMStringOrSequenceOfUnrestrictedDouble(initOrKey);
      if (typeof init === "string") {
        throw new TypeError("Unimplemented");
      }
      switch (init.length) {
        case 6:
          values = init;
          is2D = true;
          break;
        case 16:
          values = init;
          is2D = false;
          break;
        default:
          throw new TypeError("Length of matrix init sequence must be 6 or 16");
      }
    }
    if (is2D) {
      this.#m11 = values[0];
      this.#m12 = values[1];
      this.#m13 = 0;
      this.#m14 = 0;
      this.#m21 = values[2];
      this.#m22 = values[3];
      this.#m23 = 0;
      this.#m24 = 0;
      this.#m31 = 0;
      this.#m32 = 0;
      this.#m33 = 1;
      this.#m34 = 0;
      this.#m41 = values[4];
      this.#m42 = values[5];
      this.#m43 = 0;
      this.#m44 = 1;
      this.#is2D = true;
    } else {
      this.#m11 = values[0];
      this.#m12 = values[1];
      this.#m13 = values[2];
      this.#m14 = values[3];
      this.#m21 = values[4];
      this.#m22 = values[5];
      this.#m23 = values[6];
      this.#m24 = values[7];
      this.#m31 = values[8];
      this.#m32 = values[9];
      this.#m33 = values[10];
      this.#m34 = values[11];
      this.#m41 = values[12];
      this.#m42 = values[13];
      this.#m43 = values[14];
      this.#m44 = values[15];
      this.#is2D = false;
    }
  }

  static fromMatrix(other = undefined) {
    other = convertDOMMatrixInit(other);
    return createDOMMatrixReadOnlyFromDictionary(other);
  }

  static fromFloat32Array(array32) {
    const prefix =
      "Failed to execute 'fromFloat32Array' on 'DOMMatrixReadOnly'";
    requiredArguments(arguments.length, 1, prefix);
    array32 = convertFloat32Array(array32);
    switch (TypedArrayPrototypeGetLength(array32)) {
      case 6:
        return new DOMMatrixReadOnly(directConstruct, array32, true);
      case 16:
        return new DOMMatrixReadOnly(directConstruct, array32, false);
      default:
        throw new TypeError("Length of matrix init sequence must be 6 or 16");
    }
  }

  static fromFloat64Array(array64) {
    const prefix =
      "Failed to execute 'fromFloat64Array' on 'DOMMatrixReadOnly'";
    requiredArguments(arguments.length, 1, prefix);
    array64 = convertFloat64Array(array64);
    switch (TypedArrayPrototypeGetLength(array64)) {
      case 6:
        return new DOMMatrixReadOnly(directConstruct, array64, true);
      case 16:
        return new DOMMatrixReadOnly(directConstruct, array64, false);
      default:
        throw new TypeError("Length of matrix init sequence must be 6 or 16");
    }
  }

  get a() {
    return this.#m11;
  }

  get b() {
    return this.#m12;
  }

  get c() {
    return this.#m21;
  }

  get d() {
    return this.#m22;
  }

  get e() {
    return this.#m41;
  }

  get f() {
    return this.#m42;
  }

  get m11() {
    return this.#m11;
  }

  get m12() {
    return this.#m12;
  }

  get m13() {
    return this.#m13;
  }

  get m14() {
    return this.#m14;
  }

  get m21() {
    return this.#m21;
  }

  get m22() {
    return this.#m22;
  }

  get m23() {
    return this.#m23;
  }

  get m24() {
    return this.#m24;
  }

  get m31() {
    return this.#m31;
  }

  get m32() {
    return this.#m32;
  }

  get m33() {
    return this.#m33;
  }

  get m34() {
    return this.#m34;
  }

  get m41() {
    return this.#m41;
  }

  get m42() {
    return this.#m42;
  }

  get m43() {
    return this.#m43;
  }

  get m44() {
    return this.#m44;
  }

  get is2D() {
    return this.#is2D;
  }

  get #isIdentity() {
    return this.#m12 === 0 && this.#m13 === 0 && this.#m14 === 0 &&
      this.#m21 === 0 && this.#m23 === 0 && this.#m24 === 0 &&
      this.#m31 === 0 && this.#m32 === 0 && this.#m34 === 0 &&
      this.#m41 === 0 && this.#m42 === 0 && this.#m43 === 0 &&
      this.#m11 === 1 && this.#m22 === 1 && this.#m33 === 1 && this.#m44 === 1;
  }

  get isIdentity() {
    return this.#isIdentity;
  }

  translate(tx = 0, ty = 0, tz = 0) {
    this.#brand;
    tx = convertUnrestrictedDouble(tx);
    ty = convertUnrestrictedDouble(ty);
    tz = convertUnrestrictedDouble(tz);
    const result = new DOMMatrix();
    multiplyMatrices(result, this, translateTransform(tx, ty, tz));
    return result;
  }

  scale(scaleX = 1, scaleY, scaleZ = 1, originX = 0, originY = 0, originZ = 0) {
    this.#brand;
    scaleX = convertUnrestrictedDouble(scaleX);
    if (scaleY !== undefined) {
      scaleY = convertUnrestrictedDouble(scaleY);
    }
    scaleZ = convertUnrestrictedDouble(scaleZ);
    originX = convertUnrestrictedDouble(originX);
    originY = convertUnrestrictedDouble(originY);
    originZ = convertUnrestrictedDouble(originZ);
    const result = new DOMMatrix();
    multiplyMatrices(
      result,
      this,
      translateTransform(originX, originY, originZ),
    );
    multiplyMatrices(
      result,
      result,
      scaleTransform(scaleX, scaleY ?? scaleX, scaleZ),
    );
    multiplyMatrices(
      result,
      result,
      translateTransform(-originX, -originY, -originZ),
    );
    return result;
  }

  scaleNonUniform(scaleX = 1, scaleY = 1) {
    this.#brand;
    scaleX = convertUnrestrictedDouble(scaleX);
    scaleY = convertUnrestrictedDouble(scaleY);
    const result = new DOMMatrix();
    multiplyMatrices(result, this, scaleTransform(scaleX, scaleY, 1));
    return result;
  }

  scale3d(scale = 1, originX = 0, originY = 0, originZ = 0) {
    this.#brand;
    scale = convertUnrestrictedDouble(scale);
    originX = convertUnrestrictedDouble(originX);
    originY = convertUnrestrictedDouble(originY);
    originZ = convertUnrestrictedDouble(originZ);
    const result = new DOMMatrix();
    multiplyMatrices(
      result,
      this,
      translateTransform(originX, originY, originZ),
    );
    multiplyMatrices(result, result, scaleTransform(scale, scale, scale));
    multiplyMatrices(
      result,
      result,
      translateTransform(-originX, -originY, -originZ),
    );
    return result;
  }

  rotate(rotX = 0, rotY, rotZ) {
    this.#brand;
    rotX = convertUnrestrictedDouble(rotX);
    if (rotY !== undefined) {
      rotY = convertUnrestrictedDouble(rotY);
    }
    if (rotZ !== undefined) {
      rotZ = convertUnrestrictedDouble(rotZ);
    }
    if (rotY === undefined && rotZ === undefined) {
      rotZ = rotX;
      rotX = rotY = 0;
    } else {
      rotY ??= 0;
      rotZ ??= 0;
    }
    const result = new DOMMatrix();
    multiplyMatrices(result, this, rotateTransform(0, 0, 1, radians(rotZ)));
    if (rotY !== 0) {
      multiplyMatrices(result, result, rotateTransform(0, 1, 0, radians(rotY)));
    }
    if (rotX !== 0) {
      multiplyMatrices(result, result, rotateTransform(1, 0, 0, radians(rotX)));
    }
    return result;
  }

  rotateFromVector(x = 0, y = 0) {
    this.#brand;
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    const result = new DOMMatrix();
    multiplyMatrices(
      result,
      this,
      rotateTransform(0, 0, 1, x === 0 && y === 0 ? 0 : MathAtan2(y, x)),
    );
    return result;
  }

  rotateAxisAngle(x = 0, y = 0, z = 0, angle = 0) {
    this.#brand;
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    z = convertUnrestrictedDouble(z);
    angle = convertUnrestrictedDouble(angle);
    const length = MathHypot(x, y, z);
    if (length !== 0) {
      x /= length;
      y /= length;
      z /= length;
    }
    const result = new DOMMatrix();
    multiplyMatrices(result, this, rotateTransform(x, y, z, radians(angle)));
    return result;
  }

  skewX(sx = 0) {
    this.#brand;
    sx = convertUnrestrictedDouble(sx);
    const result = new DOMMatrix();
    multiplyMatrices(result, this, skewXTransform(radians(sx)));
    return result;
  }

  skewY(sy = 0) {
    this.#brand;
    sy = convertUnrestrictedDouble(sy);
    const result = new DOMMatrix();
    multiplyMatrices(result, this, skewYTransform(radians(sy)));
    return result;
  }

  multiply(other = undefined) {
    this.#brand;
    other = convertDOMMatrixInit(other);
    const result = new DOMMatrix();
    const otherObject = createDOMMatrixReadOnlyFromDictionary(other);
    multiplyMatrices(result, this, otherObject);
    return result;
  }

  flipX() {
    this.#brand;
    const result = new DOMMatrix();
    multiplyMatrices(result, this, flipXTransform);
    return result;
  }

  flipY() {
    this.#brand;
    const result = new DOMMatrix();
    multiplyMatrices(result, this, flipYTransform);
    return result;
  }

  inverse() {
    this.#brand;
    const result = new DOMMatrix();
    invertMatrix(result, this);
    return result;
  }

  transformPoint(point = undefined) {
    this.#brand;
    point = convertDOMPointInit(point);
    const pointObject = createDOMPointFromDictionary(point);
    return transformPointWithMatrix(pointObject, this);
  }

  toFloat32Array() {
    this.#brand;
    const array = new Float32Array(16);
    array[0] = this.#m11;
    array[1] = this.#m12;
    array[2] = this.#m13;
    array[3] = this.#m14;
    array[4] = this.#m21;
    array[5] = this.#m22;
    array[6] = this.#m23;
    array[7] = this.#m24;
    array[8] = this.#m31;
    array[9] = this.#m32;
    array[10] = this.#m33;
    array[11] = this.#m34;
    array[12] = this.#m41;
    array[13] = this.#m42;
    array[14] = this.#m43;
    array[15] = this.#m44;
    return array;
  }

  toFloat64Array() {
    this.#brand;
    const array = new Float64Array(16);
    array[0] = this.#m11;
    array[1] = this.#m12;
    array[2] = this.#m13;
    array[3] = this.#m14;
    array[4] = this.#m21;
    array[5] = this.#m22;
    array[6] = this.#m23;
    array[7] = this.#m24;
    array[8] = this.#m31;
    array[9] = this.#m32;
    array[10] = this.#m33;
    array[11] = this.#m34;
    array[12] = this.#m41;
    array[13] = this.#m42;
    array[14] = this.#m43;
    array[15] = this.#m44;
    return array;
  }

  toJSON() {
    this.#brand;
    return {
      a: this.#m11,
      b: this.#m12,
      c: this.#m21,
      d: this.#m22,
      e: this.#m41,
      f: this.#m42,
      m11: this.#m11,
      m12: this.#m12,
      m13: this.#m13,
      m14: this.#m14,
      m21: this.#m21,
      m22: this.#m22,
      m23: this.#m23,
      m24: this.#m24,
      m31: this.#m31,
      m32: this.#m32,
      m33: this.#m33,
      m34: this.#m34,
      m41: this.#m41,
      m42: this.#m42,
      m43: this.#m43,
      m44: this.#m44,
      is2D: this.#is2D,
      isIdentity: this.#isIdentity,
    };
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: [
          "a",
          "b",
          "c",
          "d",
          "e",
          "f",
          "m11",
          "m12",
          "m13",
          "m14",
          "m21",
          "m22",
          "m23",
          "m24",
          "m31",
          "m32",
          "m33",
          "m34",
          "m41",
          "m42",
          "m43",
          "m44",
          "is2D",
          "isIdentity",
        ],
      }),
      options,
    );
  }

  get [privateCustomInspect]() {
    try {
      return this.#inspect;
    } catch {
      return undefined;
    }
  }

  static {
    configureInterface(this);
    getDOMMatrixM11 = (o) => o.#m11;
    setDOMMatrixM11 = (o, v) => o.#m11 = v;
    getDOMMatrixM12 = (o) => o.#m12;
    setDOMMatrixM12 = (o, v) => o.#m12 = v;
    getDOMMatrixM13 = (o) => o.#m13;
    setDOMMatrixM13 = (o, v) => o.#m13 = v;
    getDOMMatrixM14 = (o) => o.#m14;
    setDOMMatrixM14 = (o, v) => o.#m14 = v;
    getDOMMatrixM21 = (o) => o.#m21;
    setDOMMatrixM21 = (o, v) => o.#m21 = v;
    getDOMMatrixM22 = (o) => o.#m22;
    setDOMMatrixM22 = (o, v) => o.#m22 = v;
    getDOMMatrixM23 = (o) => o.#m23;
    setDOMMatrixM23 = (o, v) => o.#m23 = v;
    getDOMMatrixM24 = (o) => o.#m24;
    setDOMMatrixM24 = (o, v) => o.#m24 = v;
    getDOMMatrixM31 = (o) => o.#m31;
    setDOMMatrixM31 = (o, v) => o.#m31 = v;
    getDOMMatrixM32 = (o) => o.#m32;
    setDOMMatrixM32 = (o, v) => o.#m32 = v;
    getDOMMatrixM33 = (o) => o.#m33;
    setDOMMatrixM33 = (o, v) => o.#m33 = v;
    getDOMMatrixM34 = (o) => o.#m34;
    setDOMMatrixM34 = (o, v) => o.#m34 = v;
    getDOMMatrixM41 = (o) => o.#m41;
    setDOMMatrixM41 = (o, v) => o.#m41 = v;
    getDOMMatrixM42 = (o) => o.#m42;
    setDOMMatrixM42 = (o, v) => o.#m42 = v;
    getDOMMatrixM43 = (o) => o.#m43;
    setDOMMatrixM43 = (o, v) => o.#m43 = v;
    getDOMMatrixM44 = (o) => o.#m44;
    setDOMMatrixM44 = (o, v) => o.#m44 = v;
    getDOMMatrixIs2D = (o) => o.#is2D;
    setDOMMatrixIs2D = (o, v) => o.#is2D = v;
  }
}

const translateTransform = (tx, ty, tz) =>
  tz === 0
    ? new DOMMatrixReadOnly(
      directConstruct,
      [1, 0, 0, 1, tx, ty],
      true,
    )
    : new DOMMatrixReadOnly(
      directConstruct,
      [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, tx, ty, tz, 1],
      false,
    );
const scaleTransform = (sx, sy, sz) =>
  sz === 1
    ? new DOMMatrixReadOnly(
      directConstruct,
      [sx, 0, 0, sy, 0, 0],
      true,
    )
    : new DOMMatrixReadOnly(
      directConstruct,
      [sx, 0, 0, 0, 0, sy, 0, 0, 0, 0, sz, 0, 0, 0, 0, 1],
      false,
    );
const rotateTransform = (x, y, z, alpha) => {
  const s = MathSin(alpha / 2);
  const c = MathCos(alpha / 2);
  const sc = s * c;
  const sq = s * s;
  return x === 0 && y === 0
    ? new DOMMatrixReadOnly(
      directConstruct,
      [1 - 2 * z * z * sq, 2 * z * sc, -2 * z * sc, 1 - 2 * z * z * sq, 0, 0],
      true,
    )
    : new DOMMatrixReadOnly(
      directConstruct,
      [
        1 - 2 * (y * y + z * z) * sq,
        2 * (x * y * sq + z * sc),
        2 * (x * z * sq - y * sc),
        0,
        2 * (x * y * sq - z * sc),
        1 - 2 * (x * x + z * z) * sq,
        2 * (y * z * sq + x * sc),
        0,
        2 * (x * z * sq + y * sc),
        2 * (y * z * sq - x * sc),
        1 - 2 * (x * x + y * y) * sq,
        0,
        0,
        0,
        0,
        1,
      ],
      false,
    );
};
const skewXTransform = (alpha) =>
  new DOMMatrixReadOnly(
    directConstruct,
    [1, 0, MathTan(alpha), 1, 0, 0],
    true,
  );
const skewYTransform = (beta) =>
  new DOMMatrixReadOnly(
    directConstruct,
    [1, MathTan(beta), 0, 1, 0, 0],
    true,
  );
const flipXTransform = new DOMMatrixReadOnly(
  directConstruct,
  [-1, 0, 0, 1, 0, 0],
  true,
);
const flipYTransform = new DOMMatrixReadOnly(
  directConstruct,
  [1, 0, 0, -1, 0, 0],
  true,
);

export class DOMMatrix extends DOMMatrixReadOnly {
  #brand() {}

  static fromMatrix(other = undefined) {
    other = convertDOMMatrixInit(other);
    return createDOMMatrixFromDictionary(other);
  }

  static fromFloat32Array(array32) {
    const prefix = "Failed to execute 'fromFloat32Array' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    array32 = convertFloat32Array(array32);
    switch (TypedArrayPrototypeGetLength(array32)) {
      case 6:
        return new DOMMatrix(directConstruct, array32, true);
      case 16:
        return new DOMMatrix(directConstruct, array32, false);
      default:
        throw new TypeError("Length of matrix init sequence must be 6 or 16");
    }
  }

  static fromFloat64Array(array64) {
    const prefix = "Failed to execute 'fromFloat64Array' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    array64 = convertFloat64Array(array64);
    switch (TypedArrayPrototypeGetLength(array64)) {
      case 6:
        return new DOMMatrix(directConstruct, array64, true);
      case 16:
        return new DOMMatrix(directConstruct, array64, false);
      default:
        throw new TypeError("Length of matrix init sequence must be 6 or 16");
    }
  }

  get a() {
    this.#brand;
    return getDOMMatrixM11(this);
  }

  set a(value) {
    this.#brand;
    const prefix = "Failed to set 'a' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM11(this, value);
  }

  get b() {
    this.#brand;
    return getDOMMatrixM12(this);
  }

  set b(value) {
    this.#brand;
    const prefix = "Failed to set 'b' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM12(this, value);
  }

  get c() {
    this.#brand;
    return getDOMMatrixM21(this);
  }

  set c(value) {
    this.#brand;
    const prefix = "Failed to set 'c' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM21(this, value);
  }

  get d() {
    this.#brand;
    return getDOMMatrixM22(this);
  }

  set d(value) {
    this.#brand;
    const prefix = "Failed to set 'd' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM22(this, value);
  }

  get e() {
    this.#brand;
    return getDOMMatrixM41(this);
  }

  set e(value) {
    this.#brand;
    const prefix = "Failed to set 'e' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM41(this, value);
  }

  get f() {
    this.#brand;
    return getDOMMatrixM42(this);
  }

  set f(value) {
    this.#brand;
    const prefix = "Failed to set 'f' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM42(this, value);
  }

  get m11() {
    this.#brand;
    return getDOMMatrixM11(this);
  }

  set m11(value) {
    this.#brand;
    const prefix = "Failed to set 'm11' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM11(this, value);
  }

  get m12() {
    this.#brand;
    return getDOMMatrixM12(this);
  }

  set m12(value) {
    this.#brand;
    const prefix = "Failed to set 'm12' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM12(this, value);
  }

  get m13() {
    this.#brand;
    return getDOMMatrixM13(this);
  }

  set m13(value) {
    this.#brand;
    const prefix = "Failed to set 'm13' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM13(this, value);
    if (value !== 0) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m14() {
    this.#brand;
    return getDOMMatrixM14(this);
  }

  set m14(value) {
    this.#brand;
    const prefix = "Failed to set 'm14' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM14(this, value);
    if (value !== 0) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m21() {
    this.#brand;
    return getDOMMatrixM21(this);
  }

  set m21(value) {
    this.#brand;
    const prefix = "Failed to set 'm21' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM21(this, value);
  }

  get m22() {
    this.#brand;
    return getDOMMatrixM22(this);
  }

  set m22(value) {
    this.#brand;
    const prefix = "Failed to set 'm22' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM22(this, value);
  }

  get m23() {
    this.#brand;
    return getDOMMatrixM23(this);
  }

  set m23(value) {
    this.#brand;
    const prefix = "Failed to set 'm23' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM23(this, value);
    if (value !== 0) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m24() {
    this.#brand;
    return getDOMMatrixM24(this);
  }

  set m24(value) {
    this.#brand;
    const prefix = "Failed to set 'm24' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM24(this, value);
    if (value !== 0) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m31() {
    this.#brand;
    return getDOMMatrixM31(this);
  }

  set m31(value) {
    this.#brand;
    const prefix = "Failed to set 'm31' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM31(this, value);
    if (value !== 0) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m32() {
    this.#brand;
    return getDOMMatrixM32(this);
  }

  set m32(value) {
    this.#brand;
    const prefix = "Failed to set 'm32' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM32(this, value);
    if (value !== 0) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m33() {
    this.#brand;
    return getDOMMatrixM33(this);
  }

  set m33(value) {
    this.#brand;
    const prefix = "Failed to set 'm33' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM33(this, value);
    if (value !== 1) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m34() {
    this.#brand;
    return getDOMMatrixM34(this);
  }

  set m34(value) {
    this.#brand;
    const prefix = "Failed to set 'm34' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM34(this, value);
    if (value !== 0) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m41() {
    this.#brand;
    return getDOMMatrixM41(this);
  }

  set m41(value) {
    this.#brand;
    const prefix = "Failed to set 'm41' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM41(this, value);
  }

  get m42() {
    this.#brand;
    return getDOMMatrixM42(this);
  }

  set m42(value) {
    this.#brand;
    const prefix = "Failed to set 'm42' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM42(this, value);
  }

  get m43() {
    this.#brand;
    return getDOMMatrixM43(this);
  }

  set m43(value) {
    this.#brand;
    const prefix = "Failed to set 'm43' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM43(this, value);
    if (value !== 0) {
      setDOMMatrixIs2D(this, false);
    }
  }

  get m44() {
    this.#brand;
    return getDOMMatrixM44(this);
  }

  set m44(value) {
    this.#brand;
    const prefix = "Failed to set 'm44' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    setDOMMatrixM44(this, value);
    if (value !== 1) {
      setDOMMatrixIs2D(this, false);
    }
  }

  multiplySelf(other = undefined) {
    this.#brand;
    other = convertDOMMatrixInit(other);
    const otherObject = createDOMMatrixReadOnlyFromDictionary(other);
    multiplyMatrices(this, this, otherObject);
    return this;
  }

  preMultiplySelf(other = undefined) {
    this.#brand;
    other = convertDOMMatrixInit(other);
    const otherObject = createDOMMatrixReadOnlyFromDictionary(other);
    multiplyMatrices(this, otherObject, this);
    return this;
  }

  translateSelf(tx = 0, ty = 0, tz = 0) {
    this.#brand;
    tx = convertUnrestrictedDouble(tx);
    ty = convertUnrestrictedDouble(ty);
    tz = convertUnrestrictedDouble(tz);
    multiplyMatrices(this, this, translateTransform(tx, ty, tz));
    return this;
  }

  scaleSelf(
    scaleX = 1,
    scaleY,
    scaleZ = 1,
    originX = 0,
    originY = 0,
    originZ = 0,
  ) {
    this.#brand;
    scaleX = convertUnrestrictedDouble(scaleX);
    if (scaleY !== undefined) {
      scaleY = convertUnrestrictedDouble(scaleY);
    }
    scaleZ = convertUnrestrictedDouble(scaleZ);
    originX = convertUnrestrictedDouble(originX);
    originY = convertUnrestrictedDouble(originY);
    originZ = convertUnrestrictedDouble(originZ);
    multiplyMatrices(this, this, translateTransform(originX, originY, originZ));
    multiplyMatrices(
      this,
      this,
      scaleTransform(scaleX, scaleY ?? scaleX, scaleZ),
    );
    multiplyMatrices(
      this,
      this,
      translateTransform(-originX, -originY, -originZ),
    );
    return this;
  }

  scale3dSelf(scale = 1, originX = 0, originY = 0, originZ = 0) {
    this.#brand;
    scale = convertUnrestrictedDouble(scale);
    originX = convertUnrestrictedDouble(originX);
    originY = convertUnrestrictedDouble(originY);
    originZ = convertUnrestrictedDouble(originZ);
    multiplyMatrices(this, this, translateTransform(originX, originY, originZ));
    multiplyMatrices(this, this, scaleTransform(scale, scale, scale));
    multiplyMatrices(
      this,
      this,
      translateTransform(-originX, -originY, -originZ),
    );
    return this;
  }

  rotateSelf(rotX = 0, rotY, rotZ) {
    this.#brand;
    rotX = convertUnrestrictedDouble(rotX);
    if (rotY !== undefined) {
      rotY = convertUnrestrictedDouble(rotY);
    }
    if (rotZ !== undefined) {
      rotZ = convertUnrestrictedDouble(rotZ);
    }
    if (rotY === undefined && rotZ === undefined) {
      rotZ = rotX;
      rotX = rotY = 0;
    } else {
      rotY ??= 0;
      rotZ ??= 0;
    }
    multiplyMatrices(this, this, rotateTransform(0, 0, 1, radians(rotZ)));
    if (rotY !== 0) {
      multiplyMatrices(this, this, rotateTransform(0, 1, 0, radians(rotY)));
    }
    if (rotX !== 0) {
      multiplyMatrices(this, this, rotateTransform(1, 0, 0, radians(rotX)));
    }
    return this;
  }

  rotateFromVectorSelf(x = 0, y = 0) {
    this.#brand;
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    multiplyMatrices(
      this,
      this,
      rotateTransform(0, 0, 1, x === 0 && y === 0 ? 0 : MathAtan2(y, x)),
    );
    return this;
  }

  rotateAxisAngleSelf(x = 0, y = 0, z = 0, angle = 0) {
    this.#brand;
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    z = convertUnrestrictedDouble(z);
    angle = convertUnrestrictedDouble(angle);
    const length = MathHypot(x, y, z);
    if (length !== 0) {
      x /= length;
      y /= length;
      z /= length;
    }
    multiplyMatrices(this, this, rotateTransform(x, y, z, radians(angle)));
    return this;
  }

  skewXSelf(sx = 0) {
    this.#brand;
    sx = convertUnrestrictedDouble(sx);
    multiplyMatrices(this, this, skewXTransform(radians(sx)));
    return this;
  }

  skewYSelf(sy = 0) {
    this.#brand;
    sy = convertUnrestrictedDouble(sy);
    multiplyMatrices(this, this, skewYTransform(radians(sy)));
    return this;
  }

  invertSelf() {
    this.#brand;
    invertMatrix(this, this);
    return this;
  }

  static {
    configureInterface(this);
  }
}

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

const readDOMMatrixInitMembers = (value) => {
  const result = readDOMMatrix2DInitMembers(value);
  const { is2D } = value;
  if (is2D !== undefined) {
    result.is2D = convertBoolean(is2D);
  }
  const { m13 = 0 } = value;
  result.m13 = convertUnrestrictedDouble(m13);
  const { m14 = 0 } = value;
  result.m14 = convertUnrestrictedDouble(m14);
  const { m23 = 0 } = value;
  result.m23 = convertUnrestrictedDouble(m23);
  const { m24 = 0 } = value;
  result.m24 = convertUnrestrictedDouble(m24);
  const { m31 = 0 } = value;
  result.m31 = convertUnrestrictedDouble(m31);
  const { m32 = 0 } = value;
  result.m32 = convertUnrestrictedDouble(m32);
  const { m33 = 1 } = value;
  result.m33 = convertUnrestrictedDouble(m33);
  const { m34 = 0 } = value;
  result.m34 = convertUnrestrictedDouble(m34);
  const { m43 = 0 } = value;
  result.m43 = convertUnrestrictedDouble(m43);
  const { m44 = 1 } = value;
  result.m44 = convertUnrestrictedDouble(m44);
  return result;
};
const convertDOMMatrixInit = createDictionaryConverter(
  readDOMMatrixInitMembers,
);

function validateAndFixup(other) {
  validateAndFixup2D(other);
  const { m13, m14, m23, m24, m31, m32, m33, m34, m43, m44 } = other;
  const mayBe2D = (m13 === undefined || m13 === 0) &&
    (m14 === undefined || m14 === 0) && (m23 === undefined || m23 === 0) &&
    (m24 === undefined || m24 === 0) && (m31 === undefined || m31 === 0) &&
    (m32 === undefined || m32 === 0) && (m33 === undefined || m33 === 1) &&
    (m34 === undefined || m34 === 0) && (m43 === undefined || m43 === 0) &&
    (m44 === undefined || m44 === 1);
  if (other.is2D === undefined) {
    other.is2D = mayBe2D;
  } else if (other.is2D && !mayBe2D) {
    throw new TypeError("Invalid matrix");
  }
}

function getMatrixValues(other) {
  validateAndFixup(other);
  return other.is2D
    ? [
      other.m11,
      other.m12,
      other.m21,
      other.m22,
      other.m41,
      other.m42,
    ]
    : [
      other.m11,
      other.m12,
      other.m13,
      other.m14,
      other.m21,
      other.m22,
      other.m23,
      other.m24,
      other.m31,
      other.m32,
      other.m33,
      other.m34,
      other.m41,
      other.m42,
      other.m43,
      other.m44,
    ];
}

function createDOMMatrixReadOnlyFromDictionary(other) {
  const values = getMatrixValues(other);
  return new DOMMatrixReadOnly(directConstruct, values, other.is2D);
}

function createDOMMatrixFromDictionary(other) {
  const values = getMatrixValues(other);
  return new DOMMatrix(directConstruct, values, other.is2D);
}

function multiplyMatrices(out, a, b) {
  const a11 = getDOMMatrixM11(a);
  const a12 = getDOMMatrixM12(a);
  const a13 = getDOMMatrixM13(a);
  const a14 = getDOMMatrixM14(a);
  const a21 = getDOMMatrixM21(a);
  const a22 = getDOMMatrixM22(a);
  const a23 = getDOMMatrixM23(a);
  const a24 = getDOMMatrixM24(a);
  const a31 = getDOMMatrixM31(a);
  const a32 = getDOMMatrixM32(a);
  const a33 = getDOMMatrixM33(a);
  const a34 = getDOMMatrixM34(a);
  const a41 = getDOMMatrixM41(a);
  const a42 = getDOMMatrixM42(a);
  const a43 = getDOMMatrixM43(a);
  const a44 = getDOMMatrixM44(a);
  const aIs2D = getDOMMatrixIs2D(a);
  const b11 = getDOMMatrixM11(b);
  const b12 = getDOMMatrixM12(b);
  const b13 = getDOMMatrixM13(b);
  const b14 = getDOMMatrixM14(b);
  const b21 = getDOMMatrixM21(b);
  const b22 = getDOMMatrixM22(b);
  const b23 = getDOMMatrixM23(b);
  const b24 = getDOMMatrixM24(b);
  const b31 = getDOMMatrixM31(b);
  const b32 = getDOMMatrixM32(b);
  const b33 = getDOMMatrixM33(b);
  const b34 = getDOMMatrixM34(b);
  const b41 = getDOMMatrixM41(b);
  const b42 = getDOMMatrixM42(b);
  const b43 = getDOMMatrixM43(b);
  const b44 = getDOMMatrixM44(b);
  const bIs2D = getDOMMatrixIs2D(b);
  setDOMMatrixM11(out, a11 * b11 + a21 * b12 + a31 * b13 + a41 * b14);
  setDOMMatrixM12(out, a12 * b11 + a22 * b12 + a32 * b13 + a42 * b14);
  setDOMMatrixM13(out, a13 * b11 + a23 * b12 + a33 * b13 + a43 * b14);
  setDOMMatrixM14(out, a14 * b11 + a24 * b12 + a34 * b13 + a44 * b14);
  setDOMMatrixM21(out, a11 * b21 + a21 * b22 + a31 * b23 + a41 * b24);
  setDOMMatrixM22(out, a12 * b21 + a22 * b22 + a32 * b23 + a42 * b24);
  setDOMMatrixM23(out, a13 * b21 + a23 * b22 + a33 * b23 + a43 * b24);
  setDOMMatrixM24(out, a14 * b21 + a24 * b22 + a34 * b23 + a44 * b24);
  setDOMMatrixM31(out, a11 * b31 + a21 * b32 + a31 * b33 + a41 * b34);
  setDOMMatrixM32(out, a12 * b31 + a22 * b32 + a32 * b33 + a42 * b34);
  setDOMMatrixM33(out, a13 * b31 + a23 * b32 + a33 * b33 + a43 * b34);
  setDOMMatrixM34(out, a14 * b31 + a24 * b32 + a34 * b33 + a44 * b34);
  setDOMMatrixM41(out, a11 * b41 + a21 * b42 + a31 * b43 + a41 * b44);
  setDOMMatrixM42(out, a12 * b41 + a22 * b42 + a32 * b43 + a42 * b44);
  setDOMMatrixM43(out, a13 * b41 + a23 * b42 + a33 * b43 + a43 * b44);
  setDOMMatrixM44(out, a14 * b41 + a24 * b42 + a34 * b43 + a44 * b44);
  setDOMMatrixIs2D(out, aIs2D && bIs2D);
}

function invertMatrix(out, m) {
  const m11 = getDOMMatrixM11(m);
  const m12 = getDOMMatrixM12(m);
  const m13 = getDOMMatrixM13(m);
  const m14 = getDOMMatrixM14(m);
  const m21 = getDOMMatrixM21(m);
  const m22 = getDOMMatrixM22(m);
  const m23 = getDOMMatrixM23(m);
  const m24 = getDOMMatrixM24(m);
  const m31 = getDOMMatrixM31(m);
  const m32 = getDOMMatrixM32(m);
  const m33 = getDOMMatrixM33(m);
  const m34 = getDOMMatrixM34(m);
  const m41 = getDOMMatrixM41(m);
  const m42 = getDOMMatrixM42(m);
  const m43 = getDOMMatrixM43(m);
  const m44 = getDOMMatrixM44(m);
  const det = m14 * m23 * m32 * m41 - m13 * m24 * m32 * m41 -
    m14 * m22 * m33 * m41 + m12 * m24 * m33 * m41 +
    m13 * m22 * m34 * m41 - m12 * m23 * m34 * m41 -
    m14 * m23 * m31 * m42 + m13 * m24 * m31 * m42 +
    m14 * m21 * m33 * m42 - m11 * m24 * m33 * m42 -
    m13 * m21 * m34 * m42 + m11 * m23 * m34 * m42 +
    m14 * m22 * m31 * m43 - m12 * m24 * m31 * m43 -
    m14 * m21 * m32 * m43 + m11 * m24 * m32 * m43 +
    m12 * m21 * m34 * m43 - m11 * m22 * m34 * m43 -
    m13 * m22 * m31 * m44 + m12 * m23 * m31 * m44 +
    m13 * m21 * m32 * m44 - m11 * m23 * m32 * m44 -
    m12 * m21 * m33 * m44 + m11 * m22 * m33 * m44;
  if (!det) {
    setDOMMatrixM11(out, NaN);
    setDOMMatrixM12(out, NaN);
    setDOMMatrixM13(out, NaN);
    setDOMMatrixM14(out, NaN);
    setDOMMatrixM21(out, NaN);
    setDOMMatrixM22(out, NaN);
    setDOMMatrixM23(out, NaN);
    setDOMMatrixM24(out, NaN);
    setDOMMatrixM31(out, NaN);
    setDOMMatrixM32(out, NaN);
    setDOMMatrixM33(out, NaN);
    setDOMMatrixM34(out, NaN);
    setDOMMatrixM41(out, NaN);
    setDOMMatrixM42(out, NaN);
    setDOMMatrixM43(out, NaN);
    setDOMMatrixM44(out, NaN);
    setDOMMatrixIs2D(out, false);
    return;
  }
  setDOMMatrixM11(
    out,
    (m23 * m34 * m42 - m24 * m33 * m42 + m24 * m32 * m43 -
      m22 * m34 * m43 - m23 * m32 * m44 + m22 * m33 * m44) / det,
  );
  setDOMMatrixM12(
    out,
    (m14 * m33 * m42 - m13 * m34 * m42 - m14 * m32 * m43 +
      m12 * m34 * m43 + m13 * m32 * m44 - m12 * m33 * m44) / det,
  );
  setDOMMatrixM13(
    out,
    (m13 * m24 * m42 - m14 * m23 * m42 + m14 * m22 * m43 -
      m12 * m24 * m43 - m13 * m22 * m44 + m12 * m23 * m44) / det,
  );
  setDOMMatrixM14(
    out,
    (m14 * m23 * m32 - m13 * m24 * m32 - m14 * m22 * m33 +
      m12 * m24 * m33 + m13 * m22 * m34 - m12 * m23 * m34) / det,
  );
  setDOMMatrixM21(
    out,
    (m24 * m33 * m41 - m23 * m34 * m41 - m24 * m31 * m43 +
      m21 * m34 * m43 + m23 * m31 * m44 - m21 * m33 * m44) / det,
  );
  setDOMMatrixM22(
    out,
    (m13 * m34 * m41 - m14 * m33 * m41 + m14 * m31 * m43 -
      m11 * m34 * m43 - m13 * m31 * m44 + m11 * m33 * m44) / det,
  );
  setDOMMatrixM23(
    out,
    (m14 * m23 * m41 - m13 * m24 * m41 - m14 * m21 * m43 +
      m11 * m24 * m43 + m13 * m21 * m44 - m11 * m23 * m44) / det,
  );
  setDOMMatrixM24(
    out,
    (m13 * m24 * m31 - m14 * m23 * m31 + m14 * m21 * m33 -
      m11 * m24 * m33 - m13 * m21 * m34 + m11 * m23 * m34) / det,
  );
  setDOMMatrixM31(
    out,
    (m22 * m34 * m41 - m24 * m32 * m41 + m24 * m31 * m42 -
      m21 * m34 * m42 - m22 * m31 * m44 + m21 * m32 * m44) / det,
  );
  setDOMMatrixM32(
    out,
    (m14 * m32 * m41 - m12 * m34 * m41 - m14 * m31 * m42 +
      m11 * m34 * m42 + m12 * m31 * m44 - m11 * m32 * m44) / det,
  );
  setDOMMatrixM33(
    out,
    (m12 * m24 * m41 - m14 * m22 * m41 + m14 * m21 * m42 -
      m11 * m24 * m42 - m12 * m21 * m44 + m11 * m22 * m44) / det,
  );
  setDOMMatrixM34(
    out,
    (m14 * m22 * m31 - m12 * m24 * m31 - m14 * m21 * m32 +
      m11 * m24 * m32 + m12 * m21 * m34 - m11 * m22 * m34) / det,
  );
  setDOMMatrixM41(
    out,
    (m23 * m32 * m41 - m22 * m33 * m41 - m23 * m31 * m42 +
      m21 * m33 * m42 + m22 * m31 * m43 - m21 * m32 * m43) / det,
  );
  setDOMMatrixM42(
    out,
    (m12 * m33 * m41 - m13 * m32 * m41 + m13 * m31 * m42 -
      m11 * m33 * m42 - m12 * m31 * m43 + m11 * m32 * m43) / det,
  );
  setDOMMatrixM43(
    out,
    (m13 * m22 * m41 - m12 * m23 * m41 - m13 * m21 * m42 +
      m11 * m23 * m42 + m12 * m21 * m43 - m11 * m22 * m43) / det,
  );
  setDOMMatrixM44(
    out,
    (m12 * m23 * m31 - m13 * m22 * m31 + m13 * m21 * m32 -
      m11 * m23 * m32 - m12 * m21 * m33 + m11 * m22 * m33) / det,
  );
  setDOMMatrixIs2D(out, getDOMMatrixIs2D(m));
}
