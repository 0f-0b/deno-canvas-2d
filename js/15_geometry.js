import { primordials } from "ext:core/mod.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import {
  configureInterface,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";
import { op_canvas_2d_parse_matrix } from "./00_ops.js";
import { capturePrototype } from "./01_capture_prototype.js";
import { IdentityConstructor } from "./01_identity_constructor.js";
import { sameValueZero } from "./01_same_value_zero.js";
import { createDictionaryConverter } from "./04_create_dictionary_converter.js";
import { createSequenceFromIterable } from "./04_create_sequence_from_iterable.js";
import { convertBoolean } from "./05_convert_boolean.js";
import { convertDOMString } from "./05_convert_dom_string.js";
import { convertFloat32Array } from "./05_convert_float32_array.js";
import { convertFloat64Array } from "./05_convert_float64_array.js";
import { convertUnrestrictedDouble } from "./05_convert_unrestricted_double.js";

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
  NumberIsFinite,
  Object,
  ObjectCreate,
  ObjectFreeze,
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

const DOMPointReadOnlyInternals = class DOMPointReadOnly
  extends IdentityConstructor {
  #brand() {}

  #x;
  #y;
  #z;
  #w;

  constructor(o, x, y, z, w) {
    super(o);
    this.#x = x;
    this.#y = y;
    this.#z = z;
    this.#w = w;
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getX(o) {
    return o.#x;
  }

  static setX(o, v) {
    o.#x = v;
  }

  static getY(o) {
    return o.#y;
  }

  static setY(o, v) {
    o.#y = v;
  }

  static getZ(o) {
    return o.#z;
  }

  static setZ(o, v) {
    o.#z = v;
  }

  static getW(o) {
    return o.#w;
  }

  static setW(o, v) {
    o.#w = v;
  }

  static inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["x", "y", "z", "w"],
      }),
      options,
    );
  }
};

export class DOMPointReadOnly extends Object {
  constructor(x = 0, y = 0, z = 0, w = 1) {
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    z = convertUnrestrictedDouble(z);
    w = convertUnrestrictedDouble(w);
    const newTarget = capturePrototype(new.target, DOMPointReadOnly);
    const o = ObjectCreate(newTarget.prototype);
    new DOMPointReadOnlyInternals(o, x, y, z, w);
    return o;
  }

  static fromPoint(other = undefined) {
    other = convertDOMPointInit(other);
    return createDOMPointReadOnlyFromDictionary(other);
  }

  get x() {
    return DOMPointReadOnlyInternals.getX(this);
  }

  get y() {
    return DOMPointReadOnlyInternals.getY(this);
  }

  get z() {
    return DOMPointReadOnlyInternals.getZ(this);
  }

  get w() {
    return DOMPointReadOnlyInternals.getW(this);
  }

  matrixTransform(matrix = undefined) {
    DOMPointReadOnlyInternals.checkInstance(this);
    matrix = convertDOMMatrixInit(matrix);
    const matrixObject = createDOMMatrixReadOnlyFromDictionary(matrix);
    return transformPointWithMatrix(this, matrixObject);
  }

  toJSON() {
    DOMPointReadOnlyInternals.checkInstance(this);
    return {
      x: DOMPointReadOnlyInternals.getX(this),
      y: DOMPointReadOnlyInternals.getY(this),
      z: DOMPointReadOnlyInternals.getZ(this),
      w: DOMPointReadOnlyInternals.getW(this),
    };
  }

  get [privateCustomInspect]() {
    return DOMPointReadOnlyInternals.hasInstance(this)
      ? DOMPointReadOnlyInternals.inspect
      : undefined;
  }

  static {
    configureInterface(this);
  }
}

const DOMPointInternals = class DOMPoint extends DOMPointReadOnlyInternals {
  #brand() {}

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }
};

export class DOMPoint extends DOMPointReadOnly {
  constructor(x = 0, y = 0, z = 0, w = 1) {
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    z = convertUnrestrictedDouble(z);
    w = convertUnrestrictedDouble(w);
    const newTarget = capturePrototype(new.target, DOMPoint);
    const o = ObjectCreate(newTarget.prototype);
    new DOMPointInternals(o, x, y, z, w);
    return o;
  }

  static fromPoint(other = undefined) {
    other = convertDOMPointInit(other);
    return createDOMPointFromDictionary(other);
  }

  get x() {
    DOMPointInternals.checkInstance(this);
    return DOMPointReadOnlyInternals.getX(this);
  }

  set x(value) {
    DOMPointInternals.checkInstance(this);
    const prefix = "Failed to set 'x' on 'DOMPoint'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMPointReadOnlyInternals.setX(this, value);
  }

  get y() {
    DOMPointInternals.checkInstance(this);
    return DOMPointReadOnlyInternals.getY(this);
  }

  set y(value) {
    DOMPointInternals.checkInstance(this);
    const prefix = "Failed to set 'y' on 'DOMPoint'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMPointReadOnlyInternals.setY(this, value);
  }

  get z() {
    DOMPointInternals.checkInstance(this);
    return DOMPointReadOnlyInternals.getZ(this);
  }

  set z(value) {
    DOMPointInternals.checkInstance(this);
    const prefix = "Failed to set 'z' on 'DOMPoint'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMPointReadOnlyInternals.setZ(this, value);
  }

  get w() {
    DOMPointInternals.checkInstance(this);
    return DOMPointReadOnlyInternals.getW(this);
  }

  set w(value) {
    DOMPointInternals.checkInstance(this);
    const prefix = "Failed to set 'w' on 'DOMPoint'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMPointReadOnlyInternals.setW(this, value);
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

export function createDOMPointReadOnly(x = 0, y = 0, z = 0, w = 1) {
  const o = ObjectCreate(DOMPointReadOnly.prototype);
  new DOMPointReadOnlyInternals(o, x, y, z, w);
  return o;
}

export function createDOMPoint(x = 0, y = 0, z = 0, w = 1) {
  const o = ObjectCreate(DOMPoint.prototype);
  new DOMPointInternals(o, x, y, z, w);
  return o;
}

function createDOMPointReadOnlyFromDictionary(other) {
  return createDOMPointReadOnly(other.x, other.y, other.z, other.w);
}

function createDOMPointFromDictionary(other) {
  return createDOMPoint(other.x, other.y, other.z, other.w);
}

function transformPointWithMatrix(point, matrix) {
  const x = DOMPointReadOnlyInternals.getX(point);
  const y = DOMPointReadOnlyInternals.getY(point);
  const z = DOMPointReadOnlyInternals.getZ(point);
  const w = DOMPointReadOnlyInternals.getW(point);
  const m11 = DOMMatrixReadOnlyInternals.getM11(matrix);
  const m12 = DOMMatrixReadOnlyInternals.getM12(matrix);
  const m13 = DOMMatrixReadOnlyInternals.getM13(matrix);
  const m14 = DOMMatrixReadOnlyInternals.getM14(matrix);
  const m21 = DOMMatrixReadOnlyInternals.getM21(matrix);
  const m22 = DOMMatrixReadOnlyInternals.getM22(matrix);
  const m23 = DOMMatrixReadOnlyInternals.getM23(matrix);
  const m24 = DOMMatrixReadOnlyInternals.getM24(matrix);
  const m31 = DOMMatrixReadOnlyInternals.getM31(matrix);
  const m32 = DOMMatrixReadOnlyInternals.getM32(matrix);
  const m33 = DOMMatrixReadOnlyInternals.getM33(matrix);
  const m34 = DOMMatrixReadOnlyInternals.getM34(matrix);
  const m41 = DOMMatrixReadOnlyInternals.getM41(matrix);
  const m42 = DOMMatrixReadOnlyInternals.getM42(matrix);
  const m43 = DOMMatrixReadOnlyInternals.getM43(matrix);
  const m44 = DOMMatrixReadOnlyInternals.getM44(matrix);
  return createDOMPoint(
    m11 * x + m21 * y + m31 * z + m41 * w,
    m12 * x + m22 * y + m32 * z + m42 * w,
    m13 * x + m23 * y + m33 * z + m43 * w,
    m14 * x + m24 * y + m34 * z + m44 * w,
  );
}

const DOMRectReadOnlyInternals = class DOMRectReadOnly
  extends IdentityConstructor {
  #brand() {}

  #x;
  #y;
  #width;
  #height;

  constructor(o, x, y, width, height) {
    super(o);
    this.#x = x;
    this.#y = y;
    this.#width = width;
    this.#height = height;
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getX(o) {
    return o.#x;
  }

  static setX(o, v) {
    o.#x = v;
  }

  static getY(o) {
    return o.#y;
  }

  static setY(o, v) {
    o.#y = v;
  }

  static getWidth(o) {
    return o.#width;
  }

  static setWidth(o, v) {
    o.#width = v;
  }

  static getHeight(o) {
    return o.#height;
  }

  static setHeight(o, v) {
    o.#height = v;
  }

  static getTop(o) {
    o.#brand;
    return MathMin(o.#y, o.#y + o.#height);
  }

  static getRight(o) {
    o.#brand;
    return MathMax(o.#x, o.#x + o.#width);
  }

  static getBottom(o) {
    o.#brand;
    return MathMax(o.#y, o.#y + o.#height);
  }

  static getLeft(o) {
    o.#brand;
    return MathMin(o.#x, o.#x + o.#width);
  }

  static inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["x", "y", "width", "height", "top", "right", "bottom", "left"],
      }),
      options,
    );
  }
};

export class DOMRectReadOnly extends Object {
  constructor(x = 0, y = 0, width = 0, height = 0) {
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    width = convertUnrestrictedDouble(width);
    height = convertUnrestrictedDouble(height);
    const newTarget = capturePrototype(new.target, DOMRectReadOnly);
    const o = ObjectCreate(newTarget.prototype);
    new DOMRectReadOnlyInternals(o, x, y, width, height);
    return o;
  }

  static fromRect(other = undefined) {
    other = convertDOMRectInit(other);
    return createDOMRectReadOnlyFromDictionary(other);
  }

  get x() {
    return DOMRectReadOnlyInternals.getX(this);
  }

  get y() {
    return DOMRectReadOnlyInternals.getY(this);
  }

  get width() {
    return DOMRectReadOnlyInternals.getWidth(this);
  }

  get height() {
    return DOMRectReadOnlyInternals.getHeight(this);
  }

  get top() {
    return DOMRectReadOnlyInternals.getTop(this);
  }

  get right() {
    return DOMRectReadOnlyInternals.getRight(this);
  }

  get bottom() {
    return DOMRectReadOnlyInternals.getBottom(this);
  }

  get left() {
    return DOMRectReadOnlyInternals.getLeft(this);
  }

  toJSON() {
    DOMRectReadOnlyInternals.checkInstance(this);
    return {
      x: DOMRectReadOnlyInternals.getX(this),
      y: DOMRectReadOnlyInternals.getY(this),
      width: DOMRectReadOnlyInternals.getWidth(this),
      height: DOMRectReadOnlyInternals.getHeight(this),
      top: DOMRectReadOnlyInternals.getTop(this),
      right: DOMRectReadOnlyInternals.getRight(this),
      bottom: DOMRectReadOnlyInternals.getBottom(this),
      left: DOMRectReadOnlyInternals.getLeft(this),
    };
  }

  get [privateCustomInspect]() {
    return DOMRectReadOnlyInternals.hasInstance(this)
      ? DOMRectReadOnlyInternals.inspect
      : undefined;
  }

  static {
    configureInterface(this);
  }
}

const DOMRectInternals = class DOMRect extends DOMRectReadOnlyInternals {
  #brand() {}

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }
};

export class DOMRect extends DOMRectReadOnly {
  constructor(x = 0, y = 0, width = 0, height = 0) {
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    width = convertUnrestrictedDouble(width);
    height = convertUnrestrictedDouble(height);
    const newTarget = capturePrototype(new.target, DOMRectReadOnly);
    const o = ObjectCreate(newTarget.prototype);
    new DOMRectInternals(o, x, y, width, height);
    return o;
  }

  static fromRect(other = undefined) {
    other = convertDOMRectInit(other);
    return createDOMRectFromDictionary(other);
  }

  get x() {
    DOMRectInternals.checkInstance(this);
    return DOMRectReadOnlyInternals.getX(this);
  }

  set x(value) {
    DOMRectInternals.checkInstance(this);
    const prefix = "Failed to set 'x' on 'DOMRect'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMRectReadOnlyInternals.setX(this, value);
  }

  get y() {
    DOMRectInternals.checkInstance(this);
    return DOMRectReadOnlyInternals.getY(this);
  }

  set y(value) {
    DOMRectInternals.checkInstance(this);
    const prefix = "Failed to set 'y' on 'DOMRect'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMRectReadOnlyInternals.setY(this, value);
  }

  get width() {
    DOMRectInternals.checkInstance(this);
    return DOMRectReadOnlyInternals.getWidth(this);
  }

  set width(value) {
    DOMRectInternals.checkInstance(this);
    const prefix = "Failed to set 'width' on 'DOMRect'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMRectReadOnlyInternals.setWidth(this, value);
  }

  get height() {
    DOMRectInternals.checkInstance(this);
    return DOMRectReadOnlyInternals.getHeight(this);
  }

  set height(value) {
    DOMRectInternals.checkInstance(this);
    const prefix = "Failed to set 'height' on 'DOMRect'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMRectReadOnlyInternals.setHeight(this, value);
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

export function createDOMRectReadOnly(x = 0, y = 0, width = 0, height = 0) {
  const o = ObjectCreate(DOMRectReadOnly.prototype);
  new DOMRectReadOnlyInternals(o, x, y, width, height);
  return o;
}

export function createDOMRect(x = 0, y = 0, width = 0, height = 0) {
  const o = ObjectCreate(DOMRect.prototype);
  new DOMRectInternals(o, x, y, width, height);
  return o;
}

function createDOMRectReadOnlyFromDictionary(other) {
  return createDOMRectReadOnly(other.x, other.y, other.width, other.height);
}

function createDOMRectFromDictionary(other) {
  return createDOMRect(other.x, other.y, other.width, other.height);
}

const DOMQuadInternals = class DOMQuad extends IdentityConstructor {
  #brand() {}

  #p1;
  #p2;
  #p3;
  #p4;

  constructor(o, p1, p2, p3, p4) {
    super(o);
    this.#p1 = p1;
    this.#p2 = p2;
    this.#p3 = p3;
    this.#p4 = p4;
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getP1(o) {
    return o.#p1;
  }

  static getP2(o) {
    return o.#p2;
  }

  static getP3(o) {
    return o.#p3;
  }

  static getP4(o) {
    return o.#p4;
  }

  static inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["p1", "p2", "p3", "p4"],
      }),
      options,
    );
  }
};

export class DOMQuad extends Object {
  constructor(p1 = undefined, p2, p3, p4) {
    p1 = convertDOMPointInit(p1);
    p2 = convertDOMPointInit(p2);
    p3 = convertDOMPointInit(p3);
    p4 = convertDOMPointInit(p4);
    const newTarget = capturePrototype(new.target, DOMQuad);
    const o = ObjectCreate(newTarget.prototype);
    new DOMQuadInternals(
      o,
      createDOMPointFromDictionary(p1),
      createDOMPointFromDictionary(p2),
      createDOMPointFromDictionary(p3),
      createDOMPointFromDictionary(p4),
    );
    return o;
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
    return DOMQuadInternals.getP1(this);
  }

  get p2() {
    return DOMQuadInternals.getP2(this);
  }

  get p3() {
    return DOMQuadInternals.getP3(this);
  }

  get p4() {
    return DOMQuadInternals.getP4(this);
  }

  getBounds() {
    DOMQuadInternals.checkInstance(this);
    const p1 = DOMQuadInternals.getP1(this);
    const p2 = DOMQuadInternals.getP2(this);
    const p3 = DOMQuadInternals.getP3(this);
    const p4 = DOMQuadInternals.getP4(this);
    const p1x = DOMPointReadOnlyInternals.getX(p1);
    const p2x = DOMPointReadOnlyInternals.getX(p2);
    const p3x = DOMPointReadOnlyInternals.getX(p3);
    const p4x = DOMPointReadOnlyInternals.getX(p4);
    const p1y = DOMPointReadOnlyInternals.getY(p1);
    const p2y = DOMPointReadOnlyInternals.getY(p2);
    const p3y = DOMPointReadOnlyInternals.getY(p3);
    const p4y = DOMPointReadOnlyInternals.getY(p4);
    const left = MathMin(p1x, p2x, p3x, p4x);
    const top = MathMin(p1y, p2y, p3y, p4y);
    const right = MathMax(p1x, p2x, p3x, p4x);
    const bottom = MathMax(p1y, p2y, p3y, p4y);
    return createDOMRect(left, top, right - left, bottom - top);
  }

  toJSON() {
    DOMQuadInternals.checkInstance(this);
    return {
      p1: DOMQuadInternals.getP1(this),
      p2: DOMQuadInternals.getP2(this),
      p3: DOMQuadInternals.getP3(this),
      p4: DOMQuadInternals.getP4(this),
    };
  }

  get [privateCustomInspect]() {
    return DOMQuadInternals.hasInstance(this)
      ? DOMQuadInternals.inspect
      : undefined;
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

export function createDOMQuad(
  p1 = createDOMPoint(),
  p2 = createDOMPoint(),
  p3 = createDOMPoint(),
  p4 = createDOMPoint(),
) {
  const o = ObjectCreate(DOMQuad.prototype);
  new DOMQuadInternals(o, p1, p2, p3, p4);
  return o;
}

function createDOMQuadFromDOMRectInitDictionary(other) {
  return createDOMQuad(
    createDOMPoint(other.x, other.y),
    createDOMPoint(other.x + other.width, other.y),
    createDOMPoint(other.x + other.width, other.y + other.height),
    createDOMPoint(other.x, other.y + other.height),
  );
}

function createDOMQuadFromDOMQuadInitDictionary(other) {
  return createDOMQuad(
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
const DOMMatrixReadOnlyInternals = class DOMMatrixReadOnly
  extends IdentityConstructor {
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

  constructor(o, values, is2D) {
    super(o);
    this.#init(values, is2D);
  }

  #init(values, is2D) {
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

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static init(o, values, is2D) {
    o.#init(values, is2D);
  }

  static getM11(o) {
    return o.#m11;
  }

  static setM11(o, v) {
    o.#m11 = v;
  }

  static getM12(o) {
    return o.#m12;
  }

  static setM12(o, v) {
    o.#m12 = v;
  }

  static getM13(o) {
    return o.#m13;
  }

  static setM13(o, v) {
    o.#m13 = v;
  }

  static getM14(o) {
    return o.#m14;
  }

  static setM14(o, v) {
    o.#m14 = v;
  }

  static getM21(o) {
    return o.#m21;
  }

  static setM21(o, v) {
    o.#m21 = v;
  }

  static getM22(o) {
    return o.#m22;
  }

  static setM22(o, v) {
    o.#m22 = v;
  }

  static getM23(o) {
    return o.#m23;
  }

  static setM23(o, v) {
    o.#m23 = v;
  }

  static getM24(o) {
    return o.#m24;
  }

  static setM24(o, v) {
    o.#m24 = v;
  }

  static getM31(o) {
    return o.#m31;
  }

  static setM31(o, v) {
    o.#m31 = v;
  }

  static getM32(o) {
    return o.#m32;
  }

  static setM32(o, v) {
    o.#m32 = v;
  }

  static getM33(o) {
    return o.#m33;
  }

  static setM33(o, v) {
    o.#m33 = v;
  }

  static getM34(o) {
    return o.#m34;
  }

  static setM34(o, v) {
    o.#m34 = v;
  }

  static getM41(o) {
    return o.#m41;
  }

  static setM41(o, v) {
    o.#m41 = v;
  }

  static getM42(o) {
    return o.#m42;
  }

  static setM42(o, v) {
    o.#m42 = v;
  }

  static getM43(o) {
    return o.#m43;
  }

  static setM43(o, v) {
    o.#m43 = v;
  }

  static getM44(o) {
    return o.#m44;
  }

  static setM44(o, v) {
    o.#m44 = v;
  }

  static getIs2D(o) {
    return o.#is2D;
  }

  static setIs2D(o, v) {
    o.#is2D = v;
  }

  static getIsIdentity(o) {
    o.#brand;
    return o.#m12 === 0 && o.#m13 === 0 && o.#m14 === 0 && o.#m21 === 0 &&
      o.#m23 === 0 && o.#m24 === 0 && o.#m31 === 0 && o.#m32 === 0 &&
      o.#m34 === 0 && o.#m41 === 0 && o.#m42 === 0 && o.#m43 === 0 &&
      o.#m11 === 1 && o.#m22 === 1 && o.#m33 === 1 && o.#m44 === 1;
  }

  static inspect(inspect, options) {
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
};
const identityMatrix2DValues = ObjectFreeze([1, 0, 0, 1, 0, 0]);
const parseMatrixBuffer = new Float64Array(16);

export class DOMMatrixReadOnly extends Object {
  constructor(arg = undefined) {
    const init = arg === undefined
      ? undefined
      : convertDOMStringOrSequenceOfUnrestrictedDouble(arg);
    const newTarget = capturePrototype(new.target, DOMMatrixReadOnly);
    const o = ObjectCreate(newTarget.prototype);
    let values = identityMatrix2DValues;
    let is2D = true;
    if (init !== undefined) {
      if (typeof init === "string") {
        if (inWorker) {
          throw new TypeError("Cannot construct matrix from string in workers");
        }
        values = parseMatrixBuffer;
        is2D = op_canvas_2d_parse_matrix(init, parseMatrixBuffer);
      } else {
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
            throw new TypeError(
              "Length of matrix init sequence must be 6 or 16",
            );
        }
      }
    }
    new DOMMatrixReadOnlyInternals(o, values, is2D);
    return o;
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
        return createDOMMatrixReadOnly(array32, true);
      case 16:
        return createDOMMatrixReadOnly(array32, false);
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
        return createDOMMatrixReadOnly(array64, true);
      case 16:
        return createDOMMatrixReadOnly(array64, false);
      default:
        throw new TypeError("Length of matrix init sequence must be 6 or 16");
    }
  }

  get a() {
    return DOMMatrixReadOnlyInternals.getM11(this);
  }

  get b() {
    return DOMMatrixReadOnlyInternals.getM12(this);
  }

  get c() {
    return DOMMatrixReadOnlyInternals.getM21(this);
  }

  get d() {
    return DOMMatrixReadOnlyInternals.getM22(this);
  }

  get e() {
    return DOMMatrixReadOnlyInternals.getM41(this);
  }

  get f() {
    return DOMMatrixReadOnlyInternals.getM42(this);
  }

  get m11() {
    return DOMMatrixReadOnlyInternals.getM11(this);
  }

  get m12() {
    return DOMMatrixReadOnlyInternals.getM12(this);
  }

  get m13() {
    return DOMMatrixReadOnlyInternals.getM13(this);
  }

  get m14() {
    return DOMMatrixReadOnlyInternals.getM14(this);
  }

  get m21() {
    return DOMMatrixReadOnlyInternals.getM21(this);
  }

  get m22() {
    return DOMMatrixReadOnlyInternals.getM22(this);
  }

  get m23() {
    return DOMMatrixReadOnlyInternals.getM23(this);
  }

  get m24() {
    return DOMMatrixReadOnlyInternals.getM24(this);
  }

  get m31() {
    return DOMMatrixReadOnlyInternals.getM31(this);
  }

  get m32() {
    return DOMMatrixReadOnlyInternals.getM32(this);
  }

  get m33() {
    return DOMMatrixReadOnlyInternals.getM33(this);
  }

  get m34() {
    return DOMMatrixReadOnlyInternals.getM34(this);
  }

  get m41() {
    return DOMMatrixReadOnlyInternals.getM41(this);
  }

  get m42() {
    return DOMMatrixReadOnlyInternals.getM42(this);
  }

  get m43() {
    return DOMMatrixReadOnlyInternals.getM43(this);
  }

  get m44() {
    return DOMMatrixReadOnlyInternals.getM44(this);
  }

  get is2D() {
    return DOMMatrixReadOnlyInternals.getIs2D(this);
  }

  get isIdentity() {
    return DOMMatrixReadOnlyInternals.getIsIdentity(this);
  }

  translate(tx = 0, ty = 0, tz = 0) {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    tx = convertUnrestrictedDouble(tx);
    ty = convertUnrestrictedDouble(ty);
    tz = convertUnrestrictedDouble(tz);
    const result = createDOMMatrix();
    multiplyMatrices(result, this, translateTransform(tx, ty, tz));
    return result;
  }

  scale(scaleX = 1, scaleY, scaleZ = 1, originX = 0, originY = 0, originZ = 0) {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    scaleX = convertUnrestrictedDouble(scaleX);
    if (scaleY !== undefined) {
      scaleY = convertUnrestrictedDouble(scaleY);
    }
    scaleZ = convertUnrestrictedDouble(scaleZ);
    originX = convertUnrestrictedDouble(originX);
    originY = convertUnrestrictedDouble(originY);
    originZ = convertUnrestrictedDouble(originZ);
    const result = createDOMMatrix();
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
    DOMMatrixReadOnlyInternals.checkInstance(this);
    scaleX = convertUnrestrictedDouble(scaleX);
    scaleY = convertUnrestrictedDouble(scaleY);
    const result = createDOMMatrix();
    multiplyMatrices(result, this, scaleTransform(scaleX, scaleY, 1));
    return result;
  }

  scale3d(scale = 1, originX = 0, originY = 0, originZ = 0) {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    scale = convertUnrestrictedDouble(scale);
    originX = convertUnrestrictedDouble(originX);
    originY = convertUnrestrictedDouble(originY);
    originZ = convertUnrestrictedDouble(originZ);
    const result = createDOMMatrix();
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
    DOMMatrixReadOnlyInternals.checkInstance(this);
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
    const result = createDOMMatrix();
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
    DOMMatrixReadOnlyInternals.checkInstance(this);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    const result = createDOMMatrix();
    multiplyMatrices(
      result,
      this,
      rotateTransform(0, 0, 1, x === 0 && y === 0 ? 0 : MathAtan2(y, x)),
    );
    return result;
  }

  rotateAxisAngle(x = 0, y = 0, z = 0, angle = 0) {
    DOMMatrixReadOnlyInternals.checkInstance(this);
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
    const result = createDOMMatrix();
    multiplyMatrices(result, this, rotateTransform(x, y, z, radians(angle)));
    return result;
  }

  skewX(sx = 0) {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    sx = convertUnrestrictedDouble(sx);
    const result = createDOMMatrix();
    multiplyMatrices(result, this, skewXTransform(radians(sx)));
    return result;
  }

  skewY(sy = 0) {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    sy = convertUnrestrictedDouble(sy);
    const result = createDOMMatrix();
    multiplyMatrices(result, this, skewYTransform(radians(sy)));
    return result;
  }

  multiply(other = undefined) {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    other = convertDOMMatrixInit(other);
    const result = createDOMMatrix();
    const otherObject = createDOMMatrixReadOnlyFromDictionary(other);
    multiplyMatrices(result, this, otherObject);
    return result;
  }

  flipX() {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    const result = createDOMMatrix();
    multiplyMatrices(result, this, flipXTransform);
    return result;
  }

  flipY() {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    const result = createDOMMatrix();
    multiplyMatrices(result, this, flipYTransform);
    return result;
  }

  inverse() {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    const result = createDOMMatrix();
    invertMatrix(result, this);
    return result;
  }

  transformPoint(point = undefined) {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    point = convertDOMPointInit(point);
    const pointObject = createDOMPointFromDictionary(point);
    return transformPointWithMatrix(pointObject, this);
  }

  toFloat32Array() {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    const array = new Float32Array(16);
    array[0] = DOMMatrixReadOnlyInternals.getM11(this);
    array[1] = DOMMatrixReadOnlyInternals.getM12(this);
    array[2] = DOMMatrixReadOnlyInternals.getM13(this);
    array[3] = DOMMatrixReadOnlyInternals.getM14(this);
    array[4] = DOMMatrixReadOnlyInternals.getM21(this);
    array[5] = DOMMatrixReadOnlyInternals.getM22(this);
    array[6] = DOMMatrixReadOnlyInternals.getM23(this);
    array[7] = DOMMatrixReadOnlyInternals.getM24(this);
    array[8] = DOMMatrixReadOnlyInternals.getM31(this);
    array[9] = DOMMatrixReadOnlyInternals.getM32(this);
    array[10] = DOMMatrixReadOnlyInternals.getM33(this);
    array[11] = DOMMatrixReadOnlyInternals.getM34(this);
    array[12] = DOMMatrixReadOnlyInternals.getM41(this);
    array[13] = DOMMatrixReadOnlyInternals.getM42(this);
    array[14] = DOMMatrixReadOnlyInternals.getM43(this);
    array[15] = DOMMatrixReadOnlyInternals.getM44(this);
    return array;
  }

  toFloat64Array() {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    const array = new Float64Array(16);
    array[0] = DOMMatrixReadOnlyInternals.getM11(this);
    array[1] = DOMMatrixReadOnlyInternals.getM12(this);
    array[2] = DOMMatrixReadOnlyInternals.getM13(this);
    array[3] = DOMMatrixReadOnlyInternals.getM14(this);
    array[4] = DOMMatrixReadOnlyInternals.getM21(this);
    array[5] = DOMMatrixReadOnlyInternals.getM22(this);
    array[6] = DOMMatrixReadOnlyInternals.getM23(this);
    array[7] = DOMMatrixReadOnlyInternals.getM24(this);
    array[8] = DOMMatrixReadOnlyInternals.getM31(this);
    array[9] = DOMMatrixReadOnlyInternals.getM32(this);
    array[10] = DOMMatrixReadOnlyInternals.getM33(this);
    array[11] = DOMMatrixReadOnlyInternals.getM34(this);
    array[12] = DOMMatrixReadOnlyInternals.getM41(this);
    array[13] = DOMMatrixReadOnlyInternals.getM42(this);
    array[14] = DOMMatrixReadOnlyInternals.getM43(this);
    array[15] = DOMMatrixReadOnlyInternals.getM44(this);
    return array;
  }

  toString() {
    const m11 = DOMMatrixReadOnlyInternals.getM11(this);
    const m12 = DOMMatrixReadOnlyInternals.getM12(this);
    const m13 = DOMMatrixReadOnlyInternals.getM13(this);
    const m14 = DOMMatrixReadOnlyInternals.getM14(this);
    const m21 = DOMMatrixReadOnlyInternals.getM21(this);
    const m22 = DOMMatrixReadOnlyInternals.getM22(this);
    const m23 = DOMMatrixReadOnlyInternals.getM23(this);
    const m24 = DOMMatrixReadOnlyInternals.getM24(this);
    const m31 = DOMMatrixReadOnlyInternals.getM31(this);
    const m32 = DOMMatrixReadOnlyInternals.getM32(this);
    const m33 = DOMMatrixReadOnlyInternals.getM33(this);
    const m34 = DOMMatrixReadOnlyInternals.getM34(this);
    const m41 = DOMMatrixReadOnlyInternals.getM41(this);
    const m42 = DOMMatrixReadOnlyInternals.getM42(this);
    const m43 = DOMMatrixReadOnlyInternals.getM43(this);
    const m44 = DOMMatrixReadOnlyInternals.getM44(this);
    if (
      !(NumberIsFinite(m11) && NumberIsFinite(m12) &&
        NumberIsFinite(m13) && NumberIsFinite(m14) &&
        NumberIsFinite(m21) && NumberIsFinite(m22) &&
        NumberIsFinite(m23) && NumberIsFinite(m24) &&
        NumberIsFinite(m31) && NumberIsFinite(m32) &&
        NumberIsFinite(m33) && NumberIsFinite(m34) &&
        NumberIsFinite(m41) && NumberIsFinite(m42) &&
        NumberIsFinite(m43) && NumberIsFinite(m44))
    ) {
      throw new DOMException(
        "Matrix contains non-finite values",
        "InvalidStateError",
      );
    }
    return DOMMatrixReadOnlyInternals.getIs2D(this)
      ? `matrix(${m11}, ${m12}, ${m21}, ${m22}, ${m41}, ${m42})`
      : `matrix3d(${m11}, ${m12}, ${m13}, ${m14}, ${m21}, ${m22}, ${m23}, ${m24}, ${m31}, ${m32}, ${m33}, ${m34}, ${m41}, ${m42}, ${m43}, ${m44})`;
  }

  toJSON() {
    DOMMatrixReadOnlyInternals.checkInstance(this);
    return {
      a: DOMMatrixReadOnlyInternals.getM11(this),
      b: DOMMatrixReadOnlyInternals.getM12(this),
      c: DOMMatrixReadOnlyInternals.getM21(this),
      d: DOMMatrixReadOnlyInternals.getM22(this),
      e: DOMMatrixReadOnlyInternals.getM41(this),
      f: DOMMatrixReadOnlyInternals.getM42(this),
      m11: DOMMatrixReadOnlyInternals.getM11(this),
      m12: DOMMatrixReadOnlyInternals.getM12(this),
      m13: DOMMatrixReadOnlyInternals.getM13(this),
      m14: DOMMatrixReadOnlyInternals.getM14(this),
      m21: DOMMatrixReadOnlyInternals.getM21(this),
      m22: DOMMatrixReadOnlyInternals.getM22(this),
      m23: DOMMatrixReadOnlyInternals.getM23(this),
      m24: DOMMatrixReadOnlyInternals.getM24(this),
      m31: DOMMatrixReadOnlyInternals.getM31(this),
      m32: DOMMatrixReadOnlyInternals.getM32(this),
      m33: DOMMatrixReadOnlyInternals.getM33(this),
      m34: DOMMatrixReadOnlyInternals.getM34(this),
      m41: DOMMatrixReadOnlyInternals.getM41(this),
      m42: DOMMatrixReadOnlyInternals.getM42(this),
      m43: DOMMatrixReadOnlyInternals.getM43(this),
      m44: DOMMatrixReadOnlyInternals.getM44(this),
      is2D: DOMMatrixReadOnlyInternals.getIs2D(this),
      isIdentity: DOMMatrixReadOnlyInternals.getIsIdentity(this),
    };
  }

  get [privateCustomInspect]() {
    return DOMMatrixReadOnlyInternals.hasInstance(this)
      ? DOMMatrixReadOnlyInternals.inspect
      : undefined;
  }

  static {
    configureInterface(this);
  }
}

const translateTransform = (tx, ty, tz) =>
  tz === 0
    ? createDOMMatrixReadOnly([1, 0, 0, 1, tx, ty], true)
    : createDOMMatrixReadOnly(
      [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, tx, ty, tz, 1],
      false,
    );
const scaleTransform = (sx, sy, sz) =>
  sz === 1
    ? createDOMMatrixReadOnly([sx, 0, 0, sy, 0, 0], true)
    : createDOMMatrixReadOnly(
      [sx, 0, 0, 0, 0, sy, 0, 0, 0, 0, sz, 0, 0, 0, 0, 1],
      false,
    );
const rotateTransform = (x, y, z, alpha) => {
  const s = MathSin(alpha / 2);
  const c = MathCos(alpha / 2);
  const sc = s * c;
  const sq = s * s;
  return x === 0 && y === 0
    ? createDOMMatrixReadOnly(
      [1 - 2 * z * z * sq, 2 * z * sc, -2 * z * sc, 1 - 2 * z * z * sq, 0, 0],
      true,
    )
    : createDOMMatrixReadOnly(
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
  createDOMMatrixReadOnly([1, 0, MathTan(alpha), 1, 0, 0], true);
const skewYTransform = (beta) =>
  createDOMMatrixReadOnly([1, MathTan(beta), 0, 1, 0, 0], true);
const flipXTransform = createDOMMatrixReadOnly([-1, 0, 0, 1, 0, 0], true);
const flipYTransform = createDOMMatrixReadOnly([1, 0, 0, -1, 0, 0], true);

const DOMMatrixInternals = class DOMMatrix extends DOMMatrixReadOnlyInternals {
  #brand() {}

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }
};

export class DOMMatrix extends DOMMatrixReadOnly {
  constructor(arg = undefined) {
    const init = arg === undefined
      ? undefined
      : convertDOMStringOrSequenceOfUnrestrictedDouble(arg);
    const newTarget = capturePrototype(new.target, DOMMatrix);
    const o = ObjectCreate(newTarget.prototype);
    let values = identityMatrix2DValues;
    let is2D = true;
    if (init !== undefined) {
      if (typeof init === "string") {
        if (inWorker) {
          throw new TypeError("Cannot construct matrix from string in workers");
        }
        values = parseMatrixBuffer;
        is2D = op_canvas_2d_parse_matrix(init, parseMatrixBuffer);
      } else {
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
            throw new TypeError(
              "Length of matrix init sequence must be 6 or 16",
            );
        }
      }
    }
    new DOMMatrixInternals(o, values, is2D);
    return o;
  }

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
        return createDOMMatrix(array32, true);
      case 16:
        return createDOMMatrix(array32, false);
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
        return createDOMMatrix(array64, true);
      case 16:
        return createDOMMatrix(array64, false);
      default:
        throw new TypeError("Length of matrix init sequence must be 6 or 16");
    }
  }

  get a() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM11(this);
  }

  set a(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'a' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM11(this, value);
  }

  get b() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM12(this);
  }

  set b(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'b' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM12(this, value);
  }

  get c() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM21(this);
  }

  set c(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'c' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM21(this, value);
  }

  get d() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM22(this);
  }

  set d(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'd' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM22(this, value);
  }

  get e() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM41(this);
  }

  set e(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'e' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM41(this, value);
  }

  get f() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM42(this);
  }

  set f(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'f' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM42(this, value);
  }

  get m11() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM11(this);
  }

  set m11(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm11' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM11(this, value);
  }

  get m12() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM12(this);
  }

  set m12(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm12' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM12(this, value);
  }

  get m13() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM13(this);
  }

  set m13(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm13' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM13(this, value);
    if (value !== 0) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m14() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM14(this);
  }

  set m14(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm14' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM14(this, value);
    if (value !== 0) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m21() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM21(this);
  }

  set m21(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm21' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM21(this, value);
  }

  get m22() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM22(this);
  }

  set m22(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm22' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM22(this, value);
  }

  get m23() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM23(this);
  }

  set m23(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm23' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM23(this, value);
    if (value !== 0) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m24() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM24(this);
  }

  set m24(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm24' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM24(this, value);
    if (value !== 0) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m31() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM31(this);
  }

  set m31(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm31' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM31(this, value);
    if (value !== 0) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m32() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM32(this);
  }

  set m32(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm32' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM32(this, value);
    if (value !== 0) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m33() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM33(this);
  }

  set m33(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm33' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM33(this, value);
    if (value !== 1) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m34() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM34(this);
  }

  set m34(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm34' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM34(this, value);
    if (value !== 0) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m41() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM41(this);
  }

  set m41(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm41' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM41(this, value);
  }

  get m42() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM42(this);
  }

  set m42(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm42' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM42(this, value);
  }

  get m43() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM43(this);
  }

  set m43(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm43' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM43(this, value);
    if (value !== 0) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  get m44() {
    DOMMatrixInternals.checkInstance(this);
    return DOMMatrixReadOnlyInternals.getM44(this);
  }

  set m44(value) {
    DOMMatrixInternals.checkInstance(this);
    const prefix = "Failed to set 'm44' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    DOMMatrixReadOnlyInternals.setM44(this, value);
    if (value !== 1) {
      DOMMatrixReadOnlyInternals.setIs2D(this, false);
    }
  }

  multiplySelf(other = undefined) {
    DOMMatrixInternals.checkInstance(this);
    other = convertDOMMatrixInit(other);
    const otherObject = createDOMMatrixReadOnlyFromDictionary(other);
    multiplyMatrices(this, this, otherObject);
    return this;
  }

  preMultiplySelf(other = undefined) {
    DOMMatrixInternals.checkInstance(this);
    other = convertDOMMatrixInit(other);
    const otherObject = createDOMMatrixReadOnlyFromDictionary(other);
    multiplyMatrices(this, otherObject, this);
    return this;
  }

  translateSelf(tx = 0, ty = 0, tz = 0) {
    DOMMatrixInternals.checkInstance(this);
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
    DOMMatrixInternals.checkInstance(this);
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
    DOMMatrixInternals.checkInstance(this);
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
    DOMMatrixInternals.checkInstance(this);
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
    DOMMatrixInternals.checkInstance(this);
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
    DOMMatrixInternals.checkInstance(this);
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
    DOMMatrixInternals.checkInstance(this);
    sx = convertUnrestrictedDouble(sx);
    multiplyMatrices(this, this, skewXTransform(radians(sx)));
    return this;
  }

  skewYSelf(sy = 0) {
    DOMMatrixInternals.checkInstance(this);
    sy = convertUnrestrictedDouble(sy);
    multiplyMatrices(this, this, skewYTransform(radians(sy)));
    return this;
  }

  invertSelf() {
    DOMMatrixInternals.checkInstance(this);
    invertMatrix(this, this);
    return this;
  }

  setMatrixValue(transformList) {
    const prefix = "Failed to execute 'setMatrixValue' on 'DOMMatrix'";
    requiredArguments(arguments.length, 1, prefix);
    transformList = convertDOMString(transformList);
    const is2D = op_canvas_2d_parse_matrix(transformList, parseMatrixBuffer);
    DOMMatrixReadOnlyInternals.init(this, parseMatrixBuffer, is2D);
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
  const mayBe2D = m13 === 0 && m14 === 0 && m23 === 0 && m24 === 0 &&
    m31 === 0 && m32 === 0 && m33 === 1 && m34 === 0 && m43 === 0 && m44 === 1;
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

export function createDOMMatrixReadOnly(
  values = identityMatrix2DValues,
  is2D = true,
) {
  const o = ObjectCreate(DOMMatrixReadOnly.prototype);
  new DOMMatrixReadOnlyInternals(o, values, is2D);
  return o;
}

export function createDOMMatrix(values = identityMatrix2DValues, is2D = true) {
  const o = ObjectCreate(DOMMatrix.prototype);
  new DOMMatrixInternals(o, values, is2D);
  return o;
}

function createDOMMatrixReadOnlyFromDictionary(other) {
  const values = getMatrixValues(other);
  return createDOMMatrixReadOnly(values, other.is2D);
}

function createDOMMatrixFromDictionary(other) {
  const values = getMatrixValues(other);
  return createDOMMatrix(values, other.is2D);
}

function multiplyMatrices(out, a, b) {
  const a11 = DOMMatrixReadOnlyInternals.getM11(a);
  const a12 = DOMMatrixReadOnlyInternals.getM12(a);
  const a13 = DOMMatrixReadOnlyInternals.getM13(a);
  const a14 = DOMMatrixReadOnlyInternals.getM14(a);
  const a21 = DOMMatrixReadOnlyInternals.getM21(a);
  const a22 = DOMMatrixReadOnlyInternals.getM22(a);
  const a23 = DOMMatrixReadOnlyInternals.getM23(a);
  const a24 = DOMMatrixReadOnlyInternals.getM24(a);
  const a31 = DOMMatrixReadOnlyInternals.getM31(a);
  const a32 = DOMMatrixReadOnlyInternals.getM32(a);
  const a33 = DOMMatrixReadOnlyInternals.getM33(a);
  const a34 = DOMMatrixReadOnlyInternals.getM34(a);
  const a41 = DOMMatrixReadOnlyInternals.getM41(a);
  const a42 = DOMMatrixReadOnlyInternals.getM42(a);
  const a43 = DOMMatrixReadOnlyInternals.getM43(a);
  const a44 = DOMMatrixReadOnlyInternals.getM44(a);
  const aIs2D = DOMMatrixReadOnlyInternals.getIs2D(a);
  const b11 = DOMMatrixReadOnlyInternals.getM11(b);
  const b12 = DOMMatrixReadOnlyInternals.getM12(b);
  const b13 = DOMMatrixReadOnlyInternals.getM13(b);
  const b14 = DOMMatrixReadOnlyInternals.getM14(b);
  const b21 = DOMMatrixReadOnlyInternals.getM21(b);
  const b22 = DOMMatrixReadOnlyInternals.getM22(b);
  const b23 = DOMMatrixReadOnlyInternals.getM23(b);
  const b24 = DOMMatrixReadOnlyInternals.getM24(b);
  const b31 = DOMMatrixReadOnlyInternals.getM31(b);
  const b32 = DOMMatrixReadOnlyInternals.getM32(b);
  const b33 = DOMMatrixReadOnlyInternals.getM33(b);
  const b34 = DOMMatrixReadOnlyInternals.getM34(b);
  const b41 = DOMMatrixReadOnlyInternals.getM41(b);
  const b42 = DOMMatrixReadOnlyInternals.getM42(b);
  const b43 = DOMMatrixReadOnlyInternals.getM43(b);
  const b44 = DOMMatrixReadOnlyInternals.getM44(b);
  const bIs2D = DOMMatrixReadOnlyInternals.getIs2D(b);
  DOMMatrixReadOnlyInternals.setM11(
    out,
    a11 * b11 + a21 * b12 + a31 * b13 + a41 * b14,
  );
  DOMMatrixReadOnlyInternals.setM12(
    out,
    a12 * b11 + a22 * b12 + a32 * b13 + a42 * b14,
  );
  DOMMatrixReadOnlyInternals.setM13(
    out,
    a13 * b11 + a23 * b12 + a33 * b13 + a43 * b14,
  );
  DOMMatrixReadOnlyInternals.setM14(
    out,
    a14 * b11 + a24 * b12 + a34 * b13 + a44 * b14,
  );
  DOMMatrixReadOnlyInternals.setM21(
    out,
    a11 * b21 + a21 * b22 + a31 * b23 + a41 * b24,
  );
  DOMMatrixReadOnlyInternals.setM22(
    out,
    a12 * b21 + a22 * b22 + a32 * b23 + a42 * b24,
  );
  DOMMatrixReadOnlyInternals.setM23(
    out,
    a13 * b21 + a23 * b22 + a33 * b23 + a43 * b24,
  );
  DOMMatrixReadOnlyInternals.setM24(
    out,
    a14 * b21 + a24 * b22 + a34 * b23 + a44 * b24,
  );
  DOMMatrixReadOnlyInternals.setM31(
    out,
    a11 * b31 + a21 * b32 + a31 * b33 + a41 * b34,
  );
  DOMMatrixReadOnlyInternals.setM32(
    out,
    a12 * b31 + a22 * b32 + a32 * b33 + a42 * b34,
  );
  DOMMatrixReadOnlyInternals.setM33(
    out,
    a13 * b31 + a23 * b32 + a33 * b33 + a43 * b34,
  );
  DOMMatrixReadOnlyInternals.setM34(
    out,
    a14 * b31 + a24 * b32 + a34 * b33 + a44 * b34,
  );
  DOMMatrixReadOnlyInternals.setM41(
    out,
    a11 * b41 + a21 * b42 + a31 * b43 + a41 * b44,
  );
  DOMMatrixReadOnlyInternals.setM42(
    out,
    a12 * b41 + a22 * b42 + a32 * b43 + a42 * b44,
  );
  DOMMatrixReadOnlyInternals.setM43(
    out,
    a13 * b41 + a23 * b42 + a33 * b43 + a43 * b44,
  );
  DOMMatrixReadOnlyInternals.setM44(
    out,
    a14 * b41 + a24 * b42 + a34 * b43 + a44 * b44,
  );
  DOMMatrixReadOnlyInternals.setIs2D(out, aIs2D && bIs2D);
}

function invertMatrix(out, m) {
  const m11 = DOMMatrixReadOnlyInternals.getM11(m);
  const m12 = DOMMatrixReadOnlyInternals.getM12(m);
  const m13 = DOMMatrixReadOnlyInternals.getM13(m);
  const m14 = DOMMatrixReadOnlyInternals.getM14(m);
  const m21 = DOMMatrixReadOnlyInternals.getM21(m);
  const m22 = DOMMatrixReadOnlyInternals.getM22(m);
  const m23 = DOMMatrixReadOnlyInternals.getM23(m);
  const m24 = DOMMatrixReadOnlyInternals.getM24(m);
  const m31 = DOMMatrixReadOnlyInternals.getM31(m);
  const m32 = DOMMatrixReadOnlyInternals.getM32(m);
  const m33 = DOMMatrixReadOnlyInternals.getM33(m);
  const m34 = DOMMatrixReadOnlyInternals.getM34(m);
  const m41 = DOMMatrixReadOnlyInternals.getM41(m);
  const m42 = DOMMatrixReadOnlyInternals.getM42(m);
  const m43 = DOMMatrixReadOnlyInternals.getM43(m);
  const m44 = DOMMatrixReadOnlyInternals.getM44(m);
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
    DOMMatrixReadOnlyInternals.setM11(out, NaN);
    DOMMatrixReadOnlyInternals.setM12(out, NaN);
    DOMMatrixReadOnlyInternals.setM13(out, NaN);
    DOMMatrixReadOnlyInternals.setM14(out, NaN);
    DOMMatrixReadOnlyInternals.setM21(out, NaN);
    DOMMatrixReadOnlyInternals.setM22(out, NaN);
    DOMMatrixReadOnlyInternals.setM23(out, NaN);
    DOMMatrixReadOnlyInternals.setM24(out, NaN);
    DOMMatrixReadOnlyInternals.setM31(out, NaN);
    DOMMatrixReadOnlyInternals.setM32(out, NaN);
    DOMMatrixReadOnlyInternals.setM33(out, NaN);
    DOMMatrixReadOnlyInternals.setM34(out, NaN);
    DOMMatrixReadOnlyInternals.setM41(out, NaN);
    DOMMatrixReadOnlyInternals.setM42(out, NaN);
    DOMMatrixReadOnlyInternals.setM43(out, NaN);
    DOMMatrixReadOnlyInternals.setM44(out, NaN);
    DOMMatrixReadOnlyInternals.setIs2D(out, false);
    return;
  }
  DOMMatrixReadOnlyInternals.setM11(
    out,
    (m23 * m34 * m42 - m24 * m33 * m42 + m24 * m32 * m43 -
      m22 * m34 * m43 - m23 * m32 * m44 + m22 * m33 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM12(
    out,
    (m14 * m33 * m42 - m13 * m34 * m42 - m14 * m32 * m43 +
      m12 * m34 * m43 + m13 * m32 * m44 - m12 * m33 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM13(
    out,
    (m13 * m24 * m42 - m14 * m23 * m42 + m14 * m22 * m43 -
      m12 * m24 * m43 - m13 * m22 * m44 + m12 * m23 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM14(
    out,
    (m14 * m23 * m32 - m13 * m24 * m32 - m14 * m22 * m33 +
      m12 * m24 * m33 + m13 * m22 * m34 - m12 * m23 * m34) / det,
  );
  DOMMatrixReadOnlyInternals.setM21(
    out,
    (m24 * m33 * m41 - m23 * m34 * m41 - m24 * m31 * m43 +
      m21 * m34 * m43 + m23 * m31 * m44 - m21 * m33 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM22(
    out,
    (m13 * m34 * m41 - m14 * m33 * m41 + m14 * m31 * m43 -
      m11 * m34 * m43 - m13 * m31 * m44 + m11 * m33 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM23(
    out,
    (m14 * m23 * m41 - m13 * m24 * m41 - m14 * m21 * m43 +
      m11 * m24 * m43 + m13 * m21 * m44 - m11 * m23 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM24(
    out,
    (m13 * m24 * m31 - m14 * m23 * m31 + m14 * m21 * m33 -
      m11 * m24 * m33 - m13 * m21 * m34 + m11 * m23 * m34) / det,
  );
  DOMMatrixReadOnlyInternals.setM31(
    out,
    (m22 * m34 * m41 - m24 * m32 * m41 + m24 * m31 * m42 -
      m21 * m34 * m42 - m22 * m31 * m44 + m21 * m32 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM32(
    out,
    (m14 * m32 * m41 - m12 * m34 * m41 - m14 * m31 * m42 +
      m11 * m34 * m42 + m12 * m31 * m44 - m11 * m32 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM33(
    out,
    (m12 * m24 * m41 - m14 * m22 * m41 + m14 * m21 * m42 -
      m11 * m24 * m42 - m12 * m21 * m44 + m11 * m22 * m44) / det,
  );
  DOMMatrixReadOnlyInternals.setM34(
    out,
    (m14 * m22 * m31 - m12 * m24 * m31 - m14 * m21 * m32 +
      m11 * m24 * m32 + m12 * m21 * m34 - m11 * m22 * m34) / det,
  );
  DOMMatrixReadOnlyInternals.setM41(
    out,
    (m23 * m32 * m41 - m22 * m33 * m41 - m23 * m31 * m42 +
      m21 * m33 * m42 + m22 * m31 * m43 - m21 * m32 * m43) / det,
  );
  DOMMatrixReadOnlyInternals.setM42(
    out,
    (m12 * m33 * m41 - m13 * m32 * m41 + m13 * m31 * m42 -
      m11 * m33 * m42 - m12 * m31 * m43 + m11 * m32 * m43) / det,
  );
  DOMMatrixReadOnlyInternals.setM43(
    out,
    (m13 * m22 * m41 - m12 * m23 * m41 - m13 * m21 * m42 +
      m11 * m23 * m42 + m12 * m21 * m43 - m11 * m22 * m43) / det,
  );
  DOMMatrixReadOnlyInternals.setM44(
    out,
    (m12 * m23 * m31 - m13 * m22 * m31 + m13 * m21 * m32 -
      m11 * m23 * m32 - m12 * m21 * m33 + m11 * m22 * m33) / det,
  );
  DOMMatrixReadOnlyInternals.setIs2D(
    out,
    DOMMatrixReadOnlyInternals.getIs2D(m),
  );
}

let inWorker = false;

export function initWorkerDOMMatrix() {
  inWorker = true;
  delete DOMMatrixReadOnly.prototype.toString;
  delete DOMMatrix.prototype.setMatrixValue;
}
