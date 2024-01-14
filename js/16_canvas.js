import { primordials } from "ext:core/mod.js";
import {
  ImageDataPrototypeGetColorSpace,
  ImageDataPrototypeGetData,
  ImageDataPrototypeGetHeight,
  ImageDataPrototypeGetWidth,
} from "ext:deno_canvas_2d/00_image_data_primordials.js";
import {
  op_canvas_2d_encode_png,
  op_canvas_2d_gradient_add_color_stop,
  op_canvas_2d_gradient_new_conic,
  op_canvas_2d_gradient_new_linear,
  op_canvas_2d_gradient_new_radial,
  op_canvas_2d_image_bitmap_clone,
  op_canvas_2d_image_bitmap_close,
  op_canvas_2d_image_bitmap_crop,
  op_canvas_2d_image_bitmap_empty,
  op_canvas_2d_image_bitmap_empty_resize,
  op_canvas_2d_image_bitmap_from_canvas_state,
  op_canvas_2d_image_bitmap_from_canvas_state_cropped,
  op_canvas_2d_image_bitmap_from_image_data_cropped,
  op_canvas_2d_image_bitmap_height,
  op_canvas_2d_image_bitmap_resize,
  op_canvas_2d_image_bitmap_width,
  op_canvas_2d_path_arc_to,
  op_canvas_2d_path_clear,
  op_canvas_2d_path_clone,
  op_canvas_2d_path_close,
  op_canvas_2d_path_cubic_to,
  op_canvas_2d_path_ellipse,
  op_canvas_2d_path_ensure_subpath,
  op_canvas_2d_path_extend,
  op_canvas_2d_path_line_to,
  op_canvas_2d_path_move_to,
  op_canvas_2d_path_new,
  op_canvas_2d_path_quad_to,
  op_canvas_2d_path_rect,
  op_canvas_2d_path_round_rect,
  op_canvas_2d_pattern_new,
  op_canvas_2d_pattern_set_transform,
  op_canvas_2d_state_clear,
  op_canvas_2d_state_clear_rect,
  op_canvas_2d_state_clip,
  op_canvas_2d_state_dash_list,
  op_canvas_2d_state_draw_image,
  op_canvas_2d_state_fill,
  op_canvas_2d_state_fill_rect,
  op_canvas_2d_state_fill_style,
  op_canvas_2d_state_get_image_data,
  op_canvas_2d_state_get_transform,
  op_canvas_2d_state_global_alpha,
  op_canvas_2d_state_global_composite_operation,
  op_canvas_2d_state_height,
  op_canvas_2d_state_image_smoothing_enabled,
  op_canvas_2d_state_image_smoothing_quality,
  op_canvas_2d_state_is_point_in_path,
  op_canvas_2d_state_is_point_in_stroke,
  op_canvas_2d_state_line_cap,
  op_canvas_2d_state_line_dash_offset,
  op_canvas_2d_state_line_join,
  op_canvas_2d_state_line_width,
  op_canvas_2d_state_miter_limit,
  op_canvas_2d_state_new,
  op_canvas_2d_state_put_image_data,
  op_canvas_2d_state_reset,
  op_canvas_2d_state_reset_transform,
  op_canvas_2d_state_restore,
  op_canvas_2d_state_rotate,
  op_canvas_2d_state_save,
  op_canvas_2d_state_scale,
  op_canvas_2d_state_set_dash_list,
  op_canvas_2d_state_set_fill_style_color,
  op_canvas_2d_state_set_fill_style_gradient,
  op_canvas_2d_state_set_fill_style_pattern,
  op_canvas_2d_state_set_global_alpha,
  op_canvas_2d_state_set_global_composite_operation,
  op_canvas_2d_state_set_height,
  op_canvas_2d_state_set_image_smoothing_enabled,
  op_canvas_2d_state_set_image_smoothing_quality,
  op_canvas_2d_state_set_line_cap,
  op_canvas_2d_state_set_line_dash_offset,
  op_canvas_2d_state_set_line_join,
  op_canvas_2d_state_set_line_width,
  op_canvas_2d_state_set_miter_limit,
  op_canvas_2d_state_set_shadow_blur,
  op_canvas_2d_state_set_shadow_color,
  op_canvas_2d_state_set_shadow_offset_x,
  op_canvas_2d_state_set_shadow_offset_y,
  op_canvas_2d_state_set_stroke_style_color,
  op_canvas_2d_state_set_stroke_style_gradient,
  op_canvas_2d_state_set_stroke_style_pattern,
  op_canvas_2d_state_set_transform,
  op_canvas_2d_state_set_width,
  op_canvas_2d_state_shadow_blur,
  op_canvas_2d_state_shadow_color,
  op_canvas_2d_state_shadow_offset_x,
  op_canvas_2d_state_shadow_offset_y,
  op_canvas_2d_state_stroke,
  op_canvas_2d_state_stroke_rect,
  op_canvas_2d_state_stroke_style,
  op_canvas_2d_state_transform,
  op_canvas_2d_state_translate,
  op_canvas_2d_state_width,
} from "ext:deno_canvas_2d/00_ops.js";
import { createSequenceFromIterable } from "ext:deno_canvas_2d/01_create_sequence_from_iterable.js";
import { defaultTo } from "ext:deno_canvas_2d/01_default_to.js";
import { hideSourceText } from "ext:deno_canvas_2d/01_hide_source_text.js";
import { makeSafePromise } from "ext:deno_canvas_2d/01_promise.js";
import { requireObject } from "ext:deno_canvas_2d/01_require_object.js";
import { isBlob } from "ext:deno_canvas_2d/02_is_blob.js";
import { isImageData } from "ext:deno_canvas_2d/02_is_image_data.js";
import { createDictionaryConverter } from "ext:deno_canvas_2d/04_create_dictionary_converter.js";
import {
  createEnumConverter,
  createEnumConverterForSetter,
} from "ext:deno_canvas_2d/04_create_enum_converter.js";
import { convertBoolean } from "ext:deno_canvas_2d/05_convert_boolean.js";
import { convertDOMString } from "ext:deno_canvas_2d/05_convert_dom_string.js";
import { convertDouble } from "ext:deno_canvas_2d/05_convert_double.js";
import { convertEnforceRangeLong } from "ext:deno_canvas_2d/05_convert_enforce_range_long.js";
import { convertEnforceRangeUnsignedLong } from "ext:deno_canvas_2d/05_convert_enforce_range_unsigned_long.js";
import { convertEnforceRangeUnsignedLongLong } from "ext:deno_canvas_2d/05_convert_enforce_range_unsigned_long_long.js";
import { convertEventHandler } from "ext:deno_canvas_2d/05_convert_event_handler.js";
import { convertImageData } from "ext:deno_canvas_2d/05_convert_image_data.js";
import { convertLegacyNullToEmptyStringDOMString } from "ext:deno_canvas_2d/05_convert_legacy_null_to_empty_string_dom_string.js";
import { convertLong } from "ext:deno_canvas_2d/05_convert_long.js";
import { convertUnrestrictedDouble } from "ext:deno_canvas_2d/05_convert_unrestricted_double.js";
import { EventHandler } from "ext:deno_canvas_2d/15_event.js";
import {
  convertDOMMatrix2DInit,
  convertDOMPointInit,
  directConstruct,
  DOMMatrix,
  validateAndFixup2D,
} from "ext:deno_canvas_2d/15_geometry.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import { EventTarget } from "ext:deno_web/02_event.js";
import { Blob } from "ext:deno_web/09_file.js";
import { ImageData } from "ext:deno_web/16_image_data.js";
import {
  configureInterface,
  illegalConstructor,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";

const {
  ArrayIsArray,
  ArrayPrototypeEvery,
  Float64Array,
  MathAbs,
  MathMin,
  MathSign,
  NumberIsFinite,
  ObjectFreeze,
  ObjectGetOwnPropertyDescriptors,
  ObjectGetPrototypeOf,
  Promise,
  RangeError,
  SafeArrayIterator,
  Symbol,
  SymbolFor,
  SymbolIterator,
  TypeError,
  TypedArrayPrototypeGetBuffer,
  TypedArrayPrototypeGetByteLength,
  TypedArrayPrototypeGetByteOffset,
  TypedArrayPrototypeGetLength,
  Uint32Array,
  Uint8Array,
  Uint8ClampedArray,
  globalThis,
} = primordials;
const illegalConstructorKey = Symbol();
const privateCustomInspect = SymbolFor("Deno.privateCustomInspect");
const convertCanvasImageSource = (value) => {
  if (
    type(value) === "Object" &&
    (objectIsImageBitmap(value) || objectIsOffscreenCanvas(value))
  ) {
    return value;
  }
  throw new TypeError("Expected CanvasImageSource");
};
const convertPredefinedColorSpace = createEnumConverter(
  "PredefinedColorSpace",
  ["srgb", "display-p3"],
);
const convertCanvasFillRule = createEnumConverter(
  "CanvasFillRule",
  ["nonzero", "evenodd"],
);
const readCanvasRenderingContext2DSettingsMembers = (value) => {
  const result = { __proto__: null };
  const { alpha = true } = value;
  result.alpha = convertBoolean(alpha);
  const { colorSpace = "srgb" } = value;
  result.colorSpace = convertPredefinedColorSpace(colorSpace);
  const { desynchronized = false } = value;
  result.desynchronized = convertBoolean(desynchronized);
  const { willReadFrequently = false } = value;
  result.willReadFrequently = convertBoolean(willReadFrequently);
  return result;
};
const convertCanvasRenderingContext2DSettings = createDictionaryConverter(
  readCanvasRenderingContext2DSettingsMembers,
);
const convertImageSmoothingQuality = createEnumConverterForSetter(
  ["low", "medium", "high"],
);
const convertDOMStringOrCanvasGradientOrCanvasPattern = (value) => {
  if (
    type(value) === "Object" &&
    (objectIsCanvasGradient(value) || objectIsCanvasPattern(value))
  ) {
    return value;
  }
  return convertDOMString(value);
};
const convertCanvasLineCap = createEnumConverterForSetter(
  ["butt", "round", "square"],
);
const convertCanvasLineJoin = createEnumConverterForSetter(
  ["round", "bevel", "miter"],
);
const convertCanvasTextAlign = createEnumConverterForSetter(
  ["start", "end", "left", "right", "center"],
);
const convertCanvasTextBaseline = createEnumConverterForSetter(
  ["top", "hanging", "middle", "alphabetic", "ideographic", "bottom"],
);
const convertCanvasDirection = createEnumConverterForSetter(
  ["ltr", "rtl", "inherit"],
);
const convertCanvasFontKerning = createEnumConverterForSetter(
  ["auto", "normal", "none"],
);
const convertCanvasFontStretch = createEnumConverterForSetter([
  "ultra-condensed",
  "extra-condensed",
  "condensed",
  "semi-condensed",
  "normal",
  "semi-expanded",
  "expanded",
  "extra-expanded",
  "ultra-expanded",
]);
const convertCanvasFontVariantCaps = createEnumConverterForSetter([
  "normal",
  "small-caps",
  "all-small-caps",
  "petite-caps",
  "all-petite-caps",
  "unicase",
  "titling-caps",
]);
const convertCanvasTextRendering = createEnumConverterForSetter(
  ["auto", "optimizeSpeed", "optimizeLegibility", "geometricPrecision"],
);
let objectIsCanvasGradient;
let getCanvasGradientRaw;

export class CanvasGradient {
  #brand() {}

  #raw;

  constructor(key = undefined, raw) {
    if (key !== illegalConstructorKey) {
      illegalConstructor();
    }
    this.#raw = raw;
  }

  addColorStop(offset, color) {
    this.#brand;
    const prefix = "Failed to execute 'addColorStop' on 'CanvasGradient'";
    requiredArguments(arguments.length, 2, prefix);
    offset = convertDouble(offset);
    color = convertDOMString(color);
    if (!(offset >= 0 && offset <= 1)) {
      throw new DOMException(
        "Color stop offset must be inside the range [0, 1]",
        "IndexSizeError",
      );
    }
    op_canvas_2d_gradient_add_color_stop(this.#raw, offset, color);
  }

  static {
    configureInterface(this);
    // deno-lint-ignore prefer-primordials
    objectIsCanvasGradient = (o) => #brand in o;
    getCanvasGradientRaw = (o) => o.#raw;
    hideSourceText(this);
    const proto = ObjectGetOwnPropertyDescriptors(this.prototype);
    hideSourceText(proto.addColorStop.value);
  }
}

const linearGradient = (x0, y0, x1, y1) =>
  new CanvasGradient(
    illegalConstructorKey,
    op_canvas_2d_gradient_new_linear(x0, y0, x1, y1),
  );
const radialGradient = (x0, y0, r0, x1, y1, r1) =>
  new CanvasGradient(
    illegalConstructorKey,
    op_canvas_2d_gradient_new_radial(x0, y0, r0, x1, y1, r1),
  );
const conicGradient = (startAngle, x, y) =>
  new CanvasGradient(
    illegalConstructorKey,
    op_canvas_2d_gradient_new_conic(startAngle, x, y),
  );
let objectIsCanvasPattern;
let getCanvasPatternRaw;

export class CanvasPattern {
  #brand() {}

  #raw;

  constructor(key = undefined, raw) {
    if (key !== illegalConstructorKey) {
      illegalConstructor();
    }
    this.#raw = raw;
  }

  setTransform(transform = undefined) {
    this.#brand;
    transform = convertDOMMatrix2DInit(transform);
    validateAndFixup2D(transform);
    op_canvas_2d_pattern_set_transform(
      this.#raw,
      transform.m11,
      transform.m12,
      transform.m21,
      transform.m22,
      transform.m41,
      transform.m42,
    );
  }

  static {
    configureInterface(this);
    // deno-lint-ignore prefer-primordials
    objectIsCanvasPattern = (o) => #brand in o;
    getCanvasPatternRaw = (o) => o.#raw;
    hideSourceText(this);
    const proto = ObjectGetOwnPropertyDescriptors(this.prototype);
    hideSourceText(proto.setTransform.value);
  }
}

const checkUsabilityAndClone = (image) => {
  if (objectIsImageBitmap(image)) {
    const raw = getImageBitmapRaw(image);
    if (raw === null) {
      throw new DOMException("Image is detached", "InvalidStateError");
    }
    return op_canvas_2d_image_bitmap_clone(raw);
  }
  const ctx = getOffscreenCanvasContext(image);
  if (!ctx) {
    throw new DOMException("Canvas is detached", "InvalidStateError");
  }
  const width = getOffscreenCanvasWidth(image);
  const height = getOffscreenCanvasHeight(image);
  if (width === 0 || height === 0) {
    throw new DOMException("Canvas has no pixels", "InvalidStateError");
  }
  switch (getOffscreenCanvasContextMode(ctx)) {
    case "none":
      return op_canvas_2d_image_bitmap_empty(ctx.width, ctx.height);
    case "2d": {
      const state = getOffscreenCanvasRenderingContext2DState(ctx);
      return op_canvas_2d_image_bitmap_from_canvas_state(state);
    }
  }
};
const repetitionBehaviorToRepr = ObjectFreeze({
  __proto__: null,
  "repeat": 0,
  "repeat-x": 1,
  "repeat-y": 2,
  "no-repeat": 3,
});
const getRepetitionBehavior = (repetition) => {
  const repr = repetitionBehaviorToRepr[repetition || "repeat"];
  if (repr === undefined) {
    throw new DOMException(
      `Invalid repetition mode ${repetition}`,
      "SyntaxError",
    );
  }
  return repr;
};
const pattern = (image, repetition) => {
  const bitmap = checkUsabilityAndClone(image);
  repetition = getRepetitionBehavior(repetition);
  return new CanvasPattern(
    illegalConstructorKey,
    op_canvas_2d_pattern_new(bitmap, repetition),
  );
};

export class TextMetrics {
  #width;
  #actualBoundingBoxLeft;
  #actualBoundingBoxRight;
  #fontBoundingBoxAscent;
  #fontBoundingBoxDescent;
  #actualBoundingBoxAscent;
  #actualBoundingBoxDescent;
  #emHeightAscent;
  #emHeightDescent;
  #hangingBaseline;
  #alphabeticBaseline;
  #ideographicBaseline;

  constructor(
    key = undefined,
    width,
    actualBoundingBoxLeft,
    actualBoundingBoxRight,
    fontBoundingBoxAscent,
    fontBoundingBoxDescent,
    actualBoundingBoxAscent,
    actualBoundingBoxDescent,
    emHeightAscent,
    emHeightDescent,
    hangingBaseline,
    alphabeticBaseline,
    ideographicBaseline,
  ) {
    if (key !== illegalConstructorKey) {
      illegalConstructor();
    }
    this.#width = width;
    this.#actualBoundingBoxLeft = actualBoundingBoxLeft;
    this.#actualBoundingBoxRight = actualBoundingBoxRight;
    this.#fontBoundingBoxAscent = fontBoundingBoxAscent;
    this.#fontBoundingBoxDescent = fontBoundingBoxDescent;
    this.#actualBoundingBoxAscent = actualBoundingBoxAscent;
    this.#actualBoundingBoxDescent = actualBoundingBoxDescent;
    this.#emHeightAscent = emHeightAscent;
    this.#emHeightDescent = emHeightDescent;
    this.#hangingBaseline = hangingBaseline;
    this.#alphabeticBaseline = alphabeticBaseline;
    this.#ideographicBaseline = ideographicBaseline;
  }

  get width() {
    return this.#width;
  }

  get actualBoundingBoxLeft() {
    return this.#actualBoundingBoxLeft;
  }

  get actualBoundingBoxRight() {
    return this.#actualBoundingBoxRight;
  }

  get fontBoundingBoxAscent() {
    return this.#fontBoundingBoxAscent;
  }

  get fontBoundingBoxDescent() {
    return this.#fontBoundingBoxDescent;
  }

  get actualBoundingBoxAscent() {
    return this.#actualBoundingBoxAscent;
  }

  get actualBoundingBoxDescent() {
    return this.#actualBoundingBoxDescent;
  }

  get emHeightAscent() {
    return this.#emHeightAscent;
  }

  get emHeightDescent() {
    return this.#emHeightDescent;
  }

  get hangingBaseline() {
    return this.#hangingBaseline;
  }

  get alphabeticBaseline() {
    return this.#alphabeticBaseline;
  }

  get ideographicBaseline() {
    return this.#ideographicBaseline;
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: [
          "width",
          "actualBoundingBoxLeft",
          "actualBoundingBoxRight",
          "fontBoundingBoxAscent",
          "fontBoundingBoxDescent",
          "actualBoundingBoxAscent",
          "actualBoundingBoxDescent",
          "emHeightAscent",
          "emHeightDescent",
          "hangingBaseline",
          "alphabeticBaseline",
          "ideographicBaseline",
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
    hideSourceText(this);
    const proto = ObjectGetOwnPropertyDescriptors(this.prototype);
    hideSourceText(proto.width.get);
    hideSourceText(proto.actualBoundingBoxLeft.get);
    hideSourceText(proto.actualBoundingBoxRight.get);
    hideSourceText(proto.fontBoundingBoxAscent.get);
    hideSourceText(proto.fontBoundingBoxDescent.get);
    hideSourceText(proto.actualBoundingBoxAscent.get);
    hideSourceText(proto.actualBoundingBoxDescent.get);
    hideSourceText(proto.emHeightAscent.get);
    hideSourceText(proto.emHeightDescent.get);
    hideSourceText(proto.hangingBaseline.get);
    hideSourceText(proto.alphabeticBaseline.get);
    hideSourceText(proto.ideographicBaseline.get);
  }
}

const readImageDataSettingsMembers = (value) => {
  const result = { __proto__: null };
  const { colorSpace } = value;
  if (colorSpace !== undefined) {
    result.colorSpace = convertPredefinedColorSpace(colorSpace);
  }
  return result;
};
const convertImageDataSettings = createDictionaryConverter(
  readImageDataSettingsMembers,
);
const convertPath2D = (value) => {
  if (!(type(value) === "Object" && objectIsPath2D(value))) {
    throw new TypeError("Expected Path2D");
  }
  return value;
};
const convertPath2DOrDOMString = (value) =>
  type(value) === "Object" && objectIsPath2D(value)
    ? value
    : convertDOMString(value);

function allFinite(...args) {
  return ArrayPrototypeEvery(args, NumberIsFinite);
}

function normalizeAndScaleRadii(x, y, w, h, radii) {
  if (!allFinite(x, y, w, h)) {
    return null;
  }
  if (!ArrayIsArray(radii)) {
    radii = [radii];
  }
  const count = radii.length;
  if (count < 1 || count > 4) {
    throw new RangeError("Number of radii must be between 1 and 4");
  }
  for (let i = 0; i < count; i++) {
    const radius = radii[i];
    if (typeof radius === "number") {
      if (!NumberIsFinite(radius)) {
        return null;
      }
      if (radius < 0) {
        throw new RangeError("Radius must be non-negative");
      }
      radii[i] = { x: radius, y: radius };
    } else {
      const { x, y } = radius;
      if (!allFinite(x, y)) {
        return null;
      }
      if (x < 0 || y < 0) {
        throw new RangeError("Radius must be non-negative");
      }
      radii[i] = { x, y };
    }
  }
  let upperLeft;
  let upperRight;
  let lowerRight;
  let lowerLeft;
  switch (count) {
    case 4:
      upperLeft = radii[0];
      upperRight = radii[1];
      lowerRight = radii[2];
      lowerLeft = radii[3];
      break;
    case 3:
      upperLeft = radii[0];
      upperRight = radii[1];
      lowerRight = radii[2];
      lowerLeft = radii[1];
      break;
    case 2:
      upperLeft = radii[0];
      upperRight = radii[1];
      lowerRight = radii[0];
      lowerLeft = radii[1];
      break;
    case 1:
      upperLeft = radii[0];
      upperRight = radii[0];
      lowerRight = radii[0];
      lowerLeft = radii[0];
      break;
  }
  let scale = 0;
  if (w !== 0 && h !== 0) {
    const maxX = MathAbs(w);
    const maxY = MathAbs(h);
    const top = upperLeft.x + upperRight.x;
    const right = upperRight.y + lowerRight.y;
    const bottom = lowerRight.x + lowerLeft.x;
    const left = upperLeft.y + lowerLeft.y;
    scale = MathMin(1, maxX / top, maxY / right, maxX / bottom, maxY / left);
  }
  const scaleX = scale * MathSign(w);
  const scaleY = scale * MathSign(h);
  for (let i = 0; i < count; i++) {
    const radius = radii[i];
    radius.x *= scaleX;
    radius.y *= scaleY;
  }
  return { upperLeft, upperRight, lowerRight, lowerLeft };
}

let objectIsPath2D;
let getPath2DRaw;

export class Path2D {
  #brand() {}

  #raw;

  constructor(path = undefined) {
    if (path !== undefined) {
      path = convertPath2DOrDOMString(path);
    }
    if (typeof path === "string") {
      throw new TypeError("Unimplemented");
    }
    this.#raw = path === undefined
      ? op_canvas_2d_path_new()
      : op_canvas_2d_path_clone(/** @type {Path2D} */ (path).#raw);
  }

  addPath(path, transform = undefined) {
    this.#brand;
    const prefix = "Failed to execute 'addPath' on 'Path2D'";
    requiredArguments(arguments.length, 1, prefix);
    path = convertPath2D(path);
    transform = convertDOMMatrix2DInit(transform);
    validateAndFixup2D(transform);
    op_canvas_2d_path_extend(
      this.#raw,
      path.#raw,
      transform.m11,
      transform.m12,
      transform.m21,
      transform.m22,
      transform.m41,
      transform.m42,
    );
  }

  closePath() {
    this.#brand;
    op_canvas_2d_path_close(this.#raw);
  }

  moveTo(x, y) {
    this.#brand;
    const prefix = "Failed to execute 'moveTo' on 'Path2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_move_to(this.#raw, x, y);
  }

  lineTo(x, y) {
    this.#brand;
    const prefix = "Failed to execute 'lineTo' on 'Path2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_line_to(this.#raw, x, y);
  }

  quadraticCurveTo(cpx, cpy, x, y) {
    this.#brand;
    const prefix = "Failed to execute 'quadraticCurveTo' on 'Path2D'";
    requiredArguments(arguments.length, 4, prefix);
    cpx = convertUnrestrictedDouble(cpx);
    cpy = convertUnrestrictedDouble(cpy);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_quad_to(this.#raw, cpx, cpy, x, y);
  }

  bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y) {
    this.#brand;
    const prefix = "Failed to execute 'bezierCurveTo' on 'Path2D'";
    requiredArguments(arguments.length, 6, prefix);
    cp1x = convertUnrestrictedDouble(cp1x);
    cp1y = convertUnrestrictedDouble(cp1y);
    cp2x = convertUnrestrictedDouble(cp2x);
    cp2y = convertUnrestrictedDouble(cp2y);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_cubic_to(this.#raw, cp1x, cp1y, cp2x, cp2y, x, y);
  }

  arcTo(x1, y1, x2, y2, radius) {
    this.#brand;
    const prefix = "Failed to execute 'arcTo' on 'Path2D'";
    requiredArguments(arguments.length, 5, prefix);
    x1 = convertUnrestrictedDouble(x1);
    y1 = convertUnrestrictedDouble(y1);
    x2 = convertUnrestrictedDouble(x2);
    y2 = convertUnrestrictedDouble(y2);
    radius = convertUnrestrictedDouble(radius);
    if (!allFinite(x1, y1, x2, y2, radius)) {
      return;
    }
    if (radius < 0) {
      op_canvas_2d_path_ensure_subpath(this.#raw, x1, y1);
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_arc_to(this.#raw, x1, y1, x2, y2, radius);
  }

  rect(x, y, w, h) {
    this.#brand;
    const prefix = "Failed to execute 'rect' on 'Path2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    op_canvas_2d_path_rect(this.#raw, x, y, w, h);
  }

  roundRect(x, y, w, h, radii = 0) {
    this.#brand;
    const prefix = "Failed to execute 'roundRect' on 'Path2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    radii = convertUnrestrictedDoubleOrDOMPointInitOrSequenceThereof(radii);
    const normalizedRadii = normalizeAndScaleRadii(x, y, w, h, radii);
    if (!normalizedRadii) {
      return;
    }
    const { upperLeft, upperRight, lowerRight, lowerLeft } = normalizedRadii;
    op_canvas_2d_path_round_rect(
      this.#raw,
      x,
      y,
      w,
      h,
      upperLeft.x,
      upperLeft.y,
      upperRight.x,
      upperRight.y,
      lowerRight.x,
      lowerRight.y,
      lowerLeft.x,
      lowerLeft.y,
    );
  }

  arc(x, y, radius, startAngle, endAngle, counterclockwise = false) {
    this.#brand;
    const prefix = "Failed to execute 'arc' on 'Path2D'";
    requiredArguments(arguments.length, 5, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    radius = convertUnrestrictedDouble(radius);
    startAngle = convertUnrestrictedDouble(startAngle);
    endAngle = convertUnrestrictedDouble(endAngle);
    counterclockwise = convertBoolean(counterclockwise);
    if (!allFinite(x, y, radius, startAngle, endAngle)) {
      return;
    }
    if (radius < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_ellipse(
      this.#raw,
      x,
      y,
      radius,
      radius,
      startAngle,
      endAngle,
      0,
      counterclockwise ? -1 : 1,
    );
  }

  ellipse(
    x,
    y,
    radiusX,
    radiusY,
    rotation,
    startAngle,
    endAngle,
    counterclockwise = false,
  ) {
    this.#brand;
    const prefix = "Failed to execute 'ellipse' on 'Path2D'";
    requiredArguments(arguments.length, 7, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    radiusX = convertUnrestrictedDouble(radiusX);
    radiusY = convertUnrestrictedDouble(radiusY);
    rotation = convertUnrestrictedDouble(rotation);
    startAngle = convertUnrestrictedDouble(startAngle);
    endAngle = convertUnrestrictedDouble(endAngle);
    counterclockwise = convertBoolean(counterclockwise);
    if (!allFinite(x, y, radiusX, radiusY, rotation, startAngle, endAngle)) {
      return;
    }
    if (radiusX < 0 || radiusY < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_ellipse(
      this.#raw,
      x,
      y,
      radiusX,
      radiusY,
      startAngle,
      endAngle,
      rotation,
      counterclockwise ? -1 : 1,
    );
  }

  static {
    configureInterface(this);
    // deno-lint-ignore prefer-primordials
    objectIsPath2D = (o) => #brand in o;
    getPath2DRaw = (o) => o.#raw;
    hideSourceText(this);
    const proto = ObjectGetOwnPropertyDescriptors(this.prototype);
    hideSourceText(proto.addPath.value);
    hideSourceText(proto.closePath.value);
    hideSourceText(proto.moveTo.value);
    hideSourceText(proto.lineTo.value);
    hideSourceText(proto.quadraticCurveTo.value);
    hideSourceText(proto.bezierCurveTo.value);
    hideSourceText(proto.arcTo.value);
    hideSourceText(proto.rect.value);
    hideSourceText(proto.roundRect.value);
    hideSourceText(proto.arc.value);
    hideSourceText(proto.ellipse.value);
  }
}

const readImageEncodeOptionsMembers = (value) => {
  const result = { __proto__: null };
  const { quality } = value;
  if (quality !== undefined) {
    result.quality = convertUnrestrictedDouble(quality);
  }
  const { type = "image/png" } = value;
  result.type = convertDOMString(type);
  return result;
};
const convertImageEncodeOptions = createDictionaryConverter(
  readImageEncodeOptionsMembers,
);
const convertOffscreenRenderingContextId = createEnumConverter(
  "OffscreenRenderingContextId",
  ["2d", "bitmaprenderer", "webgl", "webgl2", "webgpu"],
);
let objectIsOffscreenCanvas;
let getOffscreenCanvasContext;
let getOffscreenCanvasWidth;
let getOffscreenCanvasHeight;

function getOffscreenCanvasContextMode(ctx) {
  if (objectIsDummyCanvasContext(ctx)) {
    return "none";
  }
  if (objectIsOffscreenCanvasRenderingContext2D(ctx)) {
    return "2d";
  }
  throw new TypeError("Unreachable");
}

export class OffscreenCanvas extends EventTarget {
  #brand() {}

  #context;
  #oncontextlost = new EventHandler(this, "contextlost");
  #oncontextrestored = new EventHandler(this, "contextrestored");

  constructor(width, height) {
    const prefix = "Failed to construct 'OffscreenCanvas'";
    requiredArguments(arguments.length, 2, prefix);
    width = convertEnforceRangeUnsignedLongLong(width);
    height = convertEnforceRangeUnsignedLongLong(height);
    super();
    this.#context = new DummyCanvasContext(width, height);
  }

  // deno-lint-ignore getter-return
  get #width() {
    const ctx = this.#context;
    if (!ctx) {
      return 0;
    }
    switch (getOffscreenCanvasContextMode(ctx)) {
      case "none":
        return ctx.width;
      case "2d": {
        const state = getOffscreenCanvasRenderingContext2DState(ctx);
        return op_canvas_2d_state_width(state);
      }
    }
  }

  set #width(value) {
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    switch (getOffscreenCanvasContextMode(ctx)) {
      case "none":
        ctx.width = value;
        break;
      case "2d": {
        const state = getOffscreenCanvasRenderingContext2DState(ctx);
        op_canvas_2d_state_set_width(state, value);
        break;
      }
    }
  }

  // deno-lint-ignore getter-return
  get #height() {
    const ctx = this.#context;
    if (!ctx) {
      return 0;
    }
    switch (getOffscreenCanvasContextMode(ctx)) {
      case "none":
        return ctx.height;
      case "2d": {
        const state = getOffscreenCanvasRenderingContext2DState(ctx);
        return op_canvas_2d_state_height(state);
      }
    }
  }

  set #height(value) {
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    switch (getOffscreenCanvasContextMode(ctx)) {
      case "none":
        ctx.height = value;
        break;
      case "2d": {
        const state = getOffscreenCanvasRenderingContext2DState(ctx);
        op_canvas_2d_state_set_height(state, value);
        break;
      }
    }
  }

  get width() {
    return this.#width;
  }

  set width(value) {
    this.#brand;
    const prefix = "Failed to set 'width' on 'OffscreenCanvas'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertEnforceRangeUnsignedLongLong(value);
    this.#width = value;
  }

  get height() {
    return this.#height;
  }

  set height(value) {
    this.#brand;
    const prefix = "Failed to set 'height' on 'OffscreenCanvas'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertEnforceRangeUnsignedLongLong(value);
    this.#height = value;
  }

  getContext(contextId, options = null) {
    this.#brand;
    const prefix = "Failed to execute 'getContext' on 'OffscreenCanvas'";
    requiredArguments(arguments.length, 1, prefix);
    contextId = convertOffscreenRenderingContextId(contextId);
    if (!(typeof options === "object" || typeof options === "function")) {
      options = null;
    }
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    switch (getOffscreenCanvasContextMode(ctx)) {
      case "none": {
        const { width, height } = ctx;
        switch (contextId) {
          case "2d": {
            this.#context = new OffscreenCanvasRenderingContext2D(
              illegalConstructorKey,
              this,
              width,
              height,
              options,
            );
            break;
          }
          case "bitmaprenderer":
          case "webgl":
          case "webgl2":
          case "webgpu":
            throw new TypeError("Unimplemented");
        }
        return this.#context;
      }
      case "2d":
        return contextId === "2d" ? ctx : null;
    }
  }

  transferToImageBitmap() {
    this.#brand;
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    switch (getOffscreenCanvasContextMode(ctx)) {
      case "none":
        throw new DOMException("Canvas has no context", "InvalidStateError");
      case "2d": {
        const state = getOffscreenCanvasRenderingContext2DState(ctx);
        const image = op_canvas_2d_image_bitmap_from_canvas_state(state);
        op_canvas_2d_state_clear(state);
        return new ImageBitmap(illegalConstructorKey, image);
      }
    }
  }

  async convertToBlob(options = undefined) {
    this.#brand;
    options = convertImageEncodeOptions(options);
    try {
      const { data, type } = this.#encode(options.type);
      return new Blob(new SafeArrayIterator([data]), { __proto__: null, type });
    } finally {
      await makeSafePromise(new Promise(defer));
    }
  }

  #encode(type) {
    const { data, width, height } = this.#getAllData();
    switch (type) {
      default:
        return {
          data: op_canvas_2d_encode_png(data, width, height),
          type: "image/png",
        };
    }
  }

  #getAllData() {
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const width = this.#width;
    const height = this.#height;
    if (width === 0 || height === 0) {
      throw new DOMException("Canvas has no pixels", "IndexSizeError");
    }
    const data = new Uint8Array(width * height * 4);
    switch (getOffscreenCanvasContextMode(ctx)) {
      case "2d": {
        const state = getOffscreenCanvasRenderingContext2DState(ctx);
        const buf = new Uint32Array(TypedArrayPrototypeGetBuffer(data));
        op_canvas_2d_state_get_image_data(
          state,
          buf,
          width,
          height,
          0, // TODO support display-p3
          0,
          0,
        );
        break;
      }
    }
    return { data, width, height };
  }

  get oncontextlost() {
    return this.#oncontextlost.value;
  }

  set oncontextlost(value) {
    this.#brand;
    const prefix = "Failed to set 'oncontextlost' on 'OffscreenCanvas'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertEventHandler(value);
    this.#oncontextlost.update(value);
  }

  get oncontextrestored() {
    return this.#oncontextrestored.value;
  }

  set oncontextrestored(value) {
    this.#brand;
    const prefix = "Failed to set 'oncontextrestored' on 'OffscreenCanvas'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertEventHandler(value);
    this.#oncontextrestored.update(value);
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["width", "height", "oncontextlost", "oncontextrestored"],
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
    // deno-lint-ignore prefer-primordials
    objectIsOffscreenCanvas = (o) => #brand in o;
    getOffscreenCanvasContext = (o) => o.#context;
    getOffscreenCanvasWidth = (o) => o.#width;
    getOffscreenCanvasHeight = (o) => o.#height;
    hideSourceText(this);
    const proto = ObjectGetOwnPropertyDescriptors(this.prototype);
    hideSourceText(proto.width.get);
    hideSourceText(proto.width.set);
    hideSourceText(proto.height.get);
    hideSourceText(proto.height.set);
    hideSourceText(proto.getContext.value);
    hideSourceText(proto.transferToImageBitmap.value);
    hideSourceText(proto.convertToBlob.value);
    hideSourceText(proto.oncontextlost.get);
    hideSourceText(proto.oncontextlost.set);
    hideSourceText(proto.oncontextrestored.get);
    hideSourceText(proto.oncontextrestored.set);
  }
}

class DummyCanvasContext {
  width;
  height;

  constructor(width, height) {
    this.width = width;
    this.height = height;
  }
}

const objectIsDummyCanvasContext = (o) =>
  ObjectGetPrototypeOf(o) === DummyCanvasContext.prototype;
const convertSequenceOfUnrestrictedDouble = (value) => {
  const method = requireObject(value)[SymbolIterator];
  return createSequenceFromIterable(value, method, convertUnrestrictedDouble);
};
const convertUnrestrictedDoubleOrDOMPointInit = (value) =>
  value === null || value === undefined || type(value) === "Object"
    ? convertDOMPointInit(value)
    : convertUnrestrictedDouble(value);
const convertUnrestrictedDoubleOrDOMPointInitOrSequenceThereof = (value) => {
  if (type(value) === "Object") {
    const method = value[SymbolIterator];
    if (method !== null && method !== undefined) {
      return createSequenceFromIterable(
        value,
        method,
        convertUnrestrictedDoubleOrDOMPointInit,
      );
    }
  }
  return convertUnrestrictedDoubleOrDOMPointInit(value);
};
const alignUint8ClampedArrayToUint32 = (data) => {
  const offset = TypedArrayPrototypeGetByteOffset(data);
  const length = TypedArrayPrototypeGetByteLength(data);
  return offset % 4 === 0
    ? new Uint32Array(TypedArrayPrototypeGetBuffer(data), offset, length / 4)
    : new Uint32Array(
      TypedArrayPrototypeGetBuffer(new Uint8ClampedArray(data)),
    );
};
let objectIsOffscreenCanvasRenderingContext2D;
let getOffscreenCanvasRenderingContext2DState;
const colorSpaceToRepr = ObjectFreeze({
  __proto__: null,
  "srgb": 0,
  "display-p3": 1,
});
const lineCapFromRepr = ObjectFreeze([
  "butt",
  "round",
  "square",
]);
const lineCapToRepr = ObjectFreeze({
  __proto__: null,
  "butt": 0,
  "round": 1,
  "square": 2,
});
const lineJoinFromRepr = ObjectFreeze([
  "round",
  "bevel",
  "miter",
]);
const lineJoinToRepr = ObjectFreeze({
  __proto__: null,
  "round": 0,
  "bevel": 1,
  "miter": 2,
});
const fillRuleToRepr = ObjectFreeze({
  __proto__: null,
  "nonzero": 0,
  "evenodd": 1,
});
const blendOrCompositeModeFromRepr = ObjectFreeze([
  "normal",
  "multiply",
  "screen",
  "overlay",
  "darken",
  "lighten",
  "color-dodge",
  "color-burn",
  "hard-light",
  "soft-light",
  "difference",
  "exclusion",
  "hue",
  "saturation",
  "color",
  "luminosity",
  "clear",
  "copy",
  "source-over",
  "destination-over",
  "source-in",
  "destination-in",
  "source-out",
  "destination-out",
  "source-atop",
  "destination-atop",
  "xor",
  "lighter",
  "plus-darker",
  "plus-lighter",
]);
const blendOrCompositeModeToRepr = ObjectFreeze({
  __proto__: null,
  "normal": 0,
  "multiply": 1,
  "screen": 2,
  "overlay": 3,
  "darken": 4,
  "lighten": 5,
  "color-dodge": 6,
  "color-burn": 7,
  "hard-light": 8,
  "soft-light": 9,
  "difference": 10,
  "exclusion": 11,
  "hue": 12,
  "saturation": 13,
  "color": 14,
  "luminosity": 15,
  "clear": 16,
  "copy": 17,
  "source-over": 18,
  "destination-over": 19,
  "source-in": 20,
  "destination-in": 21,
  "source-out": 22,
  "destination-out": 23,
  "source-atop": 24,
  "destination-atop": 25,
  "xor": 26,
  "lighter": 27,
  "plus-darker": 28,
  "plus-lighter": 29,
});
const imageSmoothingQualityFromRepr = ObjectFreeze([
  "low",
  "medium",
  "high",
]);
const imageSmoothingQualityToRepr = ObjectFreeze({
  __proto__: null,
  "low": 0,
  "medium": 1,
  "high": 2,
});
const getTransformBuffer = new Float64Array(6);

export class OffscreenCanvasRenderingContext2D {
  #brand() {}

  #canvas;
  #state;
  #colorSpace;
  #cachedFillStyle = null;
  #cachedStrokeStyle = null;
  #cachedShadowColor = null;
  #cachedDefaultPath = null;

  constructor(key = undefined, target, width, height, settings) {
    if (key !== illegalConstructorKey) {
      illegalConstructor();
    }
    settings = convertCanvasRenderingContext2DSettings(settings);
    if (!settings.alpha) {
      throw new TypeError("Unsupported opaque canvas");
    }
    this.#canvas = target;
    this.#state = op_canvas_2d_state_new(
      width,
      height,
      colorSpaceToRepr[settings.colorSpace],
    );
    this.#colorSpace = settings.colorSpace;
  }

  commit() {
    this.#brand;
  }

  get canvas() {
    return this.#canvas;
  }

  save() {
    this.#brand;
    op_canvas_2d_state_save(this.#state);
  }

  restore() {
    this.#brand;
    op_canvas_2d_state_restore(this.#state);
  }

  reset() {
    this.#brand;
    op_canvas_2d_state_reset(this.#state);
  }

  isContextLost() {
    this.#brand;
    return false;
  }

  scale(x, y) {
    this.#brand;
    const prefix =
      "Failed to execute 'scale' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_state_scale(this.#state, x, y);
  }

  rotate(angle) {
    this.#brand;
    const prefix =
      "Failed to execute 'rotate' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    angle = convertUnrestrictedDouble(angle);
    op_canvas_2d_state_rotate(this.#state, angle);
  }

  translate(x, y) {
    this.#brand;
    const prefix =
      "Failed to execute 'translate' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_state_translate(this.#state, x, y);
  }

  transform(a, b, c, d, e, f) {
    this.#brand;
    const prefix =
      "Failed to execute 'transform' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 6, prefix);
    a = convertUnrestrictedDouble(a);
    b = convertUnrestrictedDouble(b);
    c = convertUnrestrictedDouble(c);
    d = convertUnrestrictedDouble(d);
    e = convertUnrestrictedDouble(e);
    f = convertUnrestrictedDouble(f);
    op_canvas_2d_state_transform(this.#state, a, b, c, d, e, f);
  }

  getTransform() {
    this.#brand;
    op_canvas_2d_state_get_transform(this.#state, getTransformBuffer);
    return new DOMMatrix(directConstruct, getTransformBuffer, true);
  }

  setTransform(a = undefined, b, c, d, e, f) {
    this.#brand;
    const nArgs = arguments.length;
    if (nArgs <= 1) {
      const matrix = convertDOMMatrix2DInit(a);
      validateAndFixup2D(matrix);
      a = matrix.m11;
      b = matrix.m12;
      c = matrix.m21;
      d = matrix.m22;
      e = matrix.m41;
      f = matrix.m42;
    } else if (nArgs >= 6) {
      a = convertUnrestrictedDouble(a);
      b = convertUnrestrictedDouble(b);
      c = convertUnrestrictedDouble(c);
      d = convertUnrestrictedDouble(d);
      e = convertUnrestrictedDouble(e);
      f = convertUnrestrictedDouble(f);
    } else {
      throw new TypeError("Overload resolution failed");
    }
    op_canvas_2d_state_set_transform(this.#state, a, b, c, d, e, f);
  }

  resetTransform() {
    this.#brand;
    op_canvas_2d_state_reset_transform(this.#state);
  }

  get globalAlpha() {
    this.#brand;
    return op_canvas_2d_state_global_alpha(this.#state);
  }

  set globalAlpha(value) {
    this.#brand;
    const prefix =
      "Failed to set 'globalAlpha' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_global_alpha(this.#state, value);
  }

  get globalCompositeOperation() {
    this.#brand;
    return blendOrCompositeModeFromRepr[
      op_canvas_2d_state_global_composite_operation(this.#state)
    ];
  }

  set globalCompositeOperation(value) {
    this.#brand;
    const prefix =
      "Failed to set 'globalCompositeOperation' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = blendOrCompositeModeToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_global_composite_operation(this.#state, repr);
  }

  get imageSmoothingEnabled() {
    this.#brand;
    return op_canvas_2d_state_image_smoothing_enabled(this.#state);
  }

  set imageSmoothingEnabled(value) {
    this.#brand;
    const prefix =
      "Failed to set 'imageSmoothingEnabled' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertBoolean(value);
    op_canvas_2d_state_set_image_smoothing_enabled(this.#state, value);
  }

  get imageSmoothingQuality() {
    this.#brand;
    return imageSmoothingQualityFromRepr[
      op_canvas_2d_state_image_smoothing_quality(this.#state)
    ];
  }

  set imageSmoothingQuality(value) {
    this.#brand;
    const prefix =
      "Failed to set 'imageSmoothingQuality' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertImageSmoothingQuality(value);
    if (value === null) {
      return;
    }
    op_canvas_2d_state_set_image_smoothing_quality(
      this.#state,
      imageSmoothingQualityToRepr[value],
    );
  }

  get strokeStyle() {
    this.#brand;
    this.#cachedStrokeStyle ??= op_canvas_2d_state_stroke_style(this.#state);
    return this.#cachedStrokeStyle;
  }

  set strokeStyle(value) {
    this.#brand;
    const prefix =
      "Failed to set 'strokeStyle' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMStringOrCanvasGradientOrCanvasPattern(value);
    if (typeof value === "string") {
      op_canvas_2d_state_set_stroke_style_color(this.#state, value);
      this.#cachedStrokeStyle = null;
    } else {
      if (objectIsCanvasGradient(value)) {
        op_canvas_2d_state_set_stroke_style_gradient(
          this.#state,
          getCanvasGradientRaw(value),
        );
      } else {
        op_canvas_2d_state_set_stroke_style_pattern(
          this.#state,
          getCanvasPatternRaw(value),
        );
      }
      this.#cachedStrokeStyle = value;
    }
  }

  get fillStyle() {
    this.#brand;
    this.#cachedFillStyle ??= op_canvas_2d_state_fill_style(this.#state);
    return this.#cachedFillStyle;
  }

  set fillStyle(value) {
    this.#brand;
    const prefix =
      "Failed to set 'fillStyle' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMStringOrCanvasGradientOrCanvasPattern(value);
    if (typeof value === "string") {
      op_canvas_2d_state_set_fill_style_color(this.#state, value);
      this.#cachedFillStyle = null;
    } else {
      if (objectIsCanvasGradient(value)) {
        op_canvas_2d_state_set_fill_style_gradient(
          this.#state,
          getCanvasGradientRaw(value),
        );
      } else {
        op_canvas_2d_state_set_fill_style_pattern(
          this.#state,
          getCanvasPatternRaw(value),
        );
      }
      this.#cachedFillStyle = value;
    }
  }

  createLinearGradient(x0, y0, x1, y1) {
    this.#brand;
    const prefix =
      "Failed to execute 'createLinearGradient' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    x0 = convertDouble(x0);
    y0 = convertDouble(y0);
    x1 = convertDouble(x1);
    y1 = convertDouble(y1);
    return linearGradient(x0, y0, x1, y1);
  }

  createRadialGradient(x0, y0, r0, x1, y1, r1) {
    this.#brand;
    const prefix =
      "Failed to execute 'createRadialGradient' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 6, prefix);
    x0 = convertDouble(x0);
    y0 = convertDouble(y0);
    r0 = convertDouble(r0);
    x1 = convertDouble(x1);
    y1 = convertDouble(y1);
    r1 = convertDouble(r1);
    if (r0 < 0 || r1 < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    return radialGradient(x0, y0, r0, x1, y1, r1);
  }

  createConicGradient(startAngle, x, y) {
    this.#brand;
    const prefix =
      "Failed to execute 'createConicGradient' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 3, prefix);
    startAngle = convertDouble(startAngle);
    x = convertDouble(x);
    y = convertDouble(y);
    return conicGradient(startAngle, x, y);
  }

  createPattern(image, repetition) {
    this.#brand;
    const prefix =
      "Failed to execute 'createPattern' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    image = convertCanvasImageSource(image);
    repetition = convertLegacyNullToEmptyStringDOMString(repetition);
    return pattern(image, repetition);
  }

  get shadowOffsetX() {
    this.#brand;
    return op_canvas_2d_state_shadow_offset_x(this.#state);
  }

  set shadowOffsetX(value) {
    this.#brand;
    const prefix =
      "Failed to set 'shadowOffsetX' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_shadow_offset_x(this.#state, value);
  }

  get shadowOffsetY() {
    this.#brand;
    return op_canvas_2d_state_shadow_offset_y(this.#state);
  }

  set shadowOffsetY(value) {
    this.#brand;
    const prefix =
      "Failed to set 'shadowOffsetY' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_shadow_offset_y(this.#state, value);
  }

  get shadowBlur() {
    this.#brand;
    return op_canvas_2d_state_shadow_blur(this.#state);
  }

  set shadowBlur(value) {
    this.#brand;
    const prefix =
      "Failed to set 'shadowBlur' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_shadow_blur(this.#state, value);
  }

  get shadowColor() {
    this.#brand;
    this.#cachedShadowColor ??= op_canvas_2d_state_shadow_color(this.#state);
    return this.#cachedShadowColor;
  }

  set shadowColor(value) {
    this.#brand;
    const prefix =
      "Failed to set 'shadowColor' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_state_set_shadow_color(this.#state, value);
    this.#cachedShadowColor = null;
  }

  get filter() {
    this.#brand;
    return "none";
  }

  set filter(value) {
    this.#brand;
    const prefix =
      "Failed to set 'filter' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  clearRect(x, y, w, h) {
    this.#brand;
    const prefix =
      "Failed to execute 'clearRect' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    if (w < 0) {
      x += w;
      w = -w;
    }
    if (h < 0) {
      y += h;
      h = -h;
    }
    op_canvas_2d_state_clear_rect(this.#state, x, y, w, h);
  }

  fillRect(x, y, w, h) {
    this.#brand;
    const prefix =
      "Failed to execute 'fillRect' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    if (w < 0) {
      x += w;
      w = -w;
    }
    if (h < 0) {
      y += h;
      h = -h;
    }
    op_canvas_2d_state_fill_rect(this.#state, x, y, w, h);
  }

  strokeRect(x, y, w, h) {
    this.#brand;
    const prefix =
      "Failed to execute 'strokeRect' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    if (w < 0) {
      x += w;
      w = -w;
    }
    if (h < 0) {
      y += h;
      h = -h;
    }
    op_canvas_2d_state_stroke_rect(this.#state, x, y, w, h);
  }

  beginPath() {
    this.#brand;
    if (this.#cachedDefaultPath) {
      op_canvas_2d_path_clear(this.#cachedDefaultPath);
    } else {
      this.#cachedDefaultPath = op_canvas_2d_path_new();
    }
  }

  get #defaultPath() {
    this.#cachedDefaultPath ??= op_canvas_2d_path_new();
    return this.#cachedDefaultPath;
  }

  #intendedPathFor(path) {
    return path ? getPath2DRaw(path) : this.#defaultPath;
  }

  fill(path = undefined, fillRule) {
    this.#brand;
    const nArgs = arguments.length;
    if (
      nArgs === 0 ||
      (nArgs === 1 && !(type(path) === "Object" && objectIsPath2D(path)))
    ) {
      fillRule = path;
      path = null;
    } else {
      path = convertPath2D(path);
    }
    fillRule = convertCanvasFillRule(defaultTo(fillRule, "nonzero"));
    op_canvas_2d_state_fill(
      this.#state,
      this.#intendedPathFor(path),
      fillRuleToRepr[fillRule],
    );
  }

  stroke(path = undefined) {
    this.#brand;
    path = arguments.length === 0 ? null : convertPath2D(path);
    op_canvas_2d_state_stroke(this.#state, this.#intendedPathFor(path));
  }

  clip(path = undefined, fillRule) {
    this.#brand;
    const nArgs = arguments.length;
    if (
      nArgs === 0 ||
      (nArgs === 1 && !(type(path) === "Object" && objectIsPath2D(path)))
    ) {
      fillRule = path;
      path = null;
    } else {
      path = convertPath2D(path);
    }
    fillRule = convertCanvasFillRule(defaultTo(fillRule, "nonzero"));
    op_canvas_2d_state_clip(
      this.#state,
      this.#intendedPathFor(path),
      fillRuleToRepr[fillRule],
    );
  }

  isPointInPath(path, x, y = undefined, fillRule) {
    this.#brand;
    const nArgs = arguments.length;
    const prefix =
      "Failed to execute 'isPointInPath' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(nArgs, 2, prefix);
    if (
      nArgs === 2 ||
      (nArgs === 3 && !(type(path) === "Object" && objectIsPath2D(path)))
    ) {
      fillRule = y;
      y = x;
      x = path;
      path = null;
    } else {
      path = convertPath2D(path);
    }
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    fillRule = convertCanvasFillRule(defaultTo(fillRule, "nonzero"));
    return op_canvas_2d_state_is_point_in_path(
      this.#state,
      this.#intendedPathFor(path),
      x,
      y,
      fillRuleToRepr[fillRule],
    );
  }

  isPointInStroke(path, x, y = undefined) {
    this.#brand;
    const nArgs = arguments.length;
    const prefix =
      "Failed to execute 'isPointInStroke' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(nArgs, 2, prefix);
    if (nArgs === 2) {
      y = x;
      x = path;
      path = null;
    } else {
      path = convertPath2D(path);
    }
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    return op_canvas_2d_state_is_point_in_stroke(
      this.#state,
      this.#intendedPathFor(path),
      x,
      y,
    );
  }

  fillText(text, x, y, maxWidth = undefined) {
    this.#brand;
    const prefix =
      "Failed to execute 'fillText' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 3, prefix);
    text = convertDOMString(text);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    if (maxWidth !== undefined) {
      maxWidth = convertUnrestrictedDouble(maxWidth);
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  strokeText(text, x, y, maxWidth = undefined) {
    this.#brand;
    const prefix =
      "Failed to execute 'strokeText' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 3, prefix);
    text = convertDOMString(text);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    if (maxWidth !== undefined) {
      maxWidth = convertUnrestrictedDouble(maxWidth);
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  measureText(text) {
    this.#brand;
    const prefix =
      "Failed to execute 'measureText' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    text = convertDOMString(text);
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  drawImage(image, sx, sy, sw = undefined, sh, dx, dy, dw, dh) {
    this.#brand;
    const nArgs = arguments.length;
    const prefix =
      "Failed to execute 'drawImage' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(nArgs, 3, prefix);
    if (nArgs === 3) {
      image = convertCanvasImageSource(image);
      dx = convertUnrestrictedDouble(sx);
      dy = convertUnrestrictedDouble(sy);
      sx = 0;
      sy = 0;
      if (!allFinite(dx, dy)) {
        return;
      }
    } else if (nArgs === 5) {
      image = convertCanvasImageSource(image);
      dx = convertUnrestrictedDouble(sx);
      dy = convertUnrestrictedDouble(sy);
      dw = convertUnrestrictedDouble(sw);
      dh = convertUnrestrictedDouble(sh);
      sx = 0;
      sy = 0;
      sw = undefined;
      sh = undefined;
      if (dw < 0) {
        dx += dw;
        dw = -dw;
      }
      if (dh < 0) {
        dy += dh;
        dh = -dh;
      }
      if (!allFinite(dx, dy, dw, dh)) {
        return;
      }
    } else if (nArgs === 9) {
      image = convertCanvasImageSource(image);
      sx = convertUnrestrictedDouble(sx);
      sy = convertUnrestrictedDouble(sy);
      sw = convertUnrestrictedDouble(sw);
      sh = convertUnrestrictedDouble(sh);
      dx = convertUnrestrictedDouble(dx);
      dy = convertUnrestrictedDouble(dy);
      dw = convertUnrestrictedDouble(dw);
      dh = convertUnrestrictedDouble(dh);
      if (sw < 0) {
        sx += sw;
        sw = -sw;
      }
      if (sh < 0) {
        sy += sh;
        sh = -sh;
      }
      if (dw < 0) {
        dx += dw;
        dw = -dw;
      }
      if (dh < 0) {
        dy += dh;
        dh = -dh;
      }
      if (!allFinite(sx, sy, sw, sh, dx, dy, dw, dh)) {
        return;
      }
    } else {
      throw new TypeError("Overload resolution failed");
    }
    const bitmap = checkUsabilityAndClone(image);
    sw ??= op_canvas_2d_image_bitmap_width(bitmap);
    sh ??= op_canvas_2d_image_bitmap_height(bitmap);
    op_canvas_2d_state_draw_image(
      this.#state,
      bitmap,
      sx,
      sy,
      sw,
      sh,
      dx,
      dy,
      dw ?? sw,
      dh ?? sh,
    );
  }

  createImageData(arg0, arg1 = undefined, arg2) {
    this.#brand;
    const nArgs = arguments.length;
    const prefix =
      "Failed to execute 'createImageData' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(nArgs, 1, prefix);
    if (nArgs === 1) {
      const imagedata = convertImageData(arg0);
      const width = ImageDataPrototypeGetWidth(imagedata);
      const height = ImageDataPrototypeGetHeight(imagedata);
      const colorSpace = ImageDataPrototypeGetColorSpace(imagedata);
      return new ImageData(width, height, { __proto__: null, colorSpace });
    } else {
      const sw = convertEnforceRangeLong(arg0);
      const sh = convertEnforceRangeLong(arg1);
      const settings = convertImageDataSettings(arg2);
      const colorSpace = settings.colorSpace ?? this.#colorSpace;
      return new ImageData(sw, sh, { __proto__: null, colorSpace });
    }
  }

  getImageData(sx, sy, sw, sh, settings = undefined) {
    this.#brand;
    const prefix =
      "Failed to execute 'getImageData' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    sx = convertEnforceRangeLong(sx);
    sy = convertEnforceRangeLong(sy);
    sw = convertEnforceRangeLong(sw);
    sh = convertEnforceRangeLong(sh);
    settings = convertImageDataSettings(settings);
    if (sw === 0) {
      throw new DOMException(
        "Source width must be non-zero",
        "IndexSizeError",
      );
    }
    if (sh === 0) {
      throw new DOMException(
        "Source height must be non-zero",
        "IndexSizeError",
      );
    }
    if (sw < 0) {
      sx += sw;
      sw = -sw;
    }
    if (sh < 0) {
      sy += sh;
      sh = -sh;
    }
    const colorSpace = settings.colorSpace ?? this.#colorSpace;
    const result = new ImageData(sw, sh, { __proto__: null, colorSpace });
    const buf = new Uint32Array(
      TypedArrayPrototypeGetBuffer(ImageDataPrototypeGetData(result)),
    );
    op_canvas_2d_state_get_image_data(
      this.#state,
      buf,
      sw,
      sh,
      colorSpaceToRepr[colorSpace],
      sx,
      sy,
    );
    return result;
  }

  putImageData(
    imagedata,
    dx,
    dy,
    dirtyX = undefined,
    dirtyY,
    dirtyWidth,
    dirtyHeight,
  ) {
    this.#brand;
    const nArgs = arguments.length;
    const prefix =
      "Failed to execute 'putImageData' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(nArgs, 3, prefix);
    let sw;
    let sh;
    if (nArgs === 3) {
      imagedata = convertImageData(imagedata);
      sw = ImageDataPrototypeGetWidth(imagedata);
      sh = ImageDataPrototypeGetHeight(imagedata);
      dx = convertEnforceRangeLong(dx);
      dy = convertEnforceRangeLong(dy);
      dirtyX = 0;
      dirtyY = 0;
      dirtyWidth = sw;
      dirtyHeight = sh;
    } else if (nArgs >= 7) {
      imagedata = convertImageData(imagedata);
      sw = ImageDataPrototypeGetWidth(imagedata);
      sh = ImageDataPrototypeGetHeight(imagedata);
      dx = convertEnforceRangeLong(dx);
      dy = convertEnforceRangeLong(dy);
      dirtyX = convertEnforceRangeLong(dirtyX);
      dirtyY = convertEnforceRangeLong(dirtyY);
      dirtyWidth = convertEnforceRangeLong(dirtyWidth);
      dirtyHeight = convertEnforceRangeLong(dirtyHeight);
      if (dirtyWidth < 0) {
        dirtyX += dirtyWidth;
        dirtyWidth = -dirtyWidth;
      }
      if (dirtyHeight < 0) {
        dirtyY += dirtyHeight;
        dirtyHeight = -dirtyHeight;
      }
      if (dirtyX < 0) {
        dirtyWidth += dirtyX;
        dirtyX = 0;
      }
      if (dirtyY < 0) {
        dirtyHeight += dirtyY;
        dirtyY = 0;
      }
      if (dirtyX + dirtyWidth > sw) {
        dirtyWidth = sw - dirtyX;
      }
      if (dirtyY + dirtyHeight > sh) {
        dirtyHeight = sh - dirtyY;
      }
    } else {
      throw new TypeError("Overload resolution failed");
    }
    const data = ImageDataPrototypeGetData(imagedata);
    if (TypedArrayPrototypeGetLength(data) === 0) {
      throw new DOMException("Image data is detached", "InvalidStateError");
    }
    if (dirtyWidth <= 0 || dirtyHeight <= 0) {
      return;
    }
    const buf = alignUint8ClampedArrayToUint32(data);
    const colorSpace = ImageDataPrototypeGetColorSpace(imagedata);
    op_canvas_2d_state_put_image_data(
      this.#state,
      buf,
      sw,
      sh,
      colorSpaceToRepr[colorSpace],
      dirtyX,
      dirtyY,
      dirtyWidth,
      dirtyHeight,
      dx + dirtyX,
      dy + dirtyY,
    );
  }

  get lineWidth() {
    this.#brand;
    return op_canvas_2d_state_line_width(this.#state);
  }

  set lineWidth(value) {
    this.#brand;
    const prefix =
      "Failed to set 'lineWidth' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_line_width(this.#state, value);
  }

  get lineCap() {
    this.#brand;
    return lineCapFromRepr[op_canvas_2d_state_line_cap(this.#state)];
  }

  set lineCap(value) {
    this.#brand;
    const prefix =
      "Failed to set 'lineCap' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasLineCap(value);
    if (value === null) {
      return;
    }
    op_canvas_2d_state_set_line_cap(this.#state, lineCapToRepr[value]);
  }

  get lineJoin() {
    this.#brand;
    return lineJoinFromRepr[op_canvas_2d_state_line_join(this.#state)];
  }

  set lineJoin(value) {
    this.#brand;
    const prefix =
      "Failed to set 'lineJoin' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasLineJoin(value);
    if (value === null) {
      return;
    }
    op_canvas_2d_state_set_line_join(this.#state, lineJoinToRepr[value]);
  }

  get miterLimit() {
    this.#brand;
    return op_canvas_2d_state_miter_limit(this.#state);
  }

  set miterLimit(value) {
    this.#brand;
    const prefix =
      "Failed to set 'miterLimit' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_miter_limit(this.#state, value);
  }

  setLineDash(segments) {
    this.#brand;
    const prefix =
      "Failed to execute 'setLineDash' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    segments = convertSequenceOfUnrestrictedDouble(segments);
    const count = segments.length;
    const buf = new Float64Array(count);
    for (let i = 0; i < count; i++) {
      const value = segments[i];
      if (!(NumberIsFinite(value) && value >= 0)) {
        return;
      }
      buf[i] = value;
    }
    op_canvas_2d_state_set_dash_list(this.#state, buf);
  }

  getLineDash() {
    this.#brand;
    return op_canvas_2d_state_dash_list(this.#state);
  }

  get lineDashOffset() {
    this.#brand;
    return op_canvas_2d_state_line_dash_offset(this.#state);
  }

  set lineDashOffset(value) {
    this.#brand;
    const prefix =
      "Failed to set 'lineDashOffset' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_line_dash_offset(this.#state, value);
  }

  get font() {
    this.#brand;
    return "10px sans-serif";
  }

  set font(value) {
    this.#brand;
    const prefix =
      "Failed to set 'font' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get textAlign() {
    this.#brand;
    return "start";
  }

  set textAlign(value) {
    this.#brand;
    const prefix =
      "Failed to set 'textAlign' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasTextAlign(value);
    if (value === null) {
      return;
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get textBaseline() {
    this.#brand;
    return "alphabetic";
  }

  set textBaseline(value) {
    this.#brand;
    const prefix =
      "Failed to set 'textBaseline' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasTextBaseline(value);
    if (value === null) {
      return;
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get direction() {
    this.#brand;
    return "inherit";
  }

  set direction(value) {
    this.#brand;
    const prefix =
      "Failed to set 'direction' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasDirection(value);
    if (value === null) {
      return;
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get letterSpacing() {
    this.#brand;
    return "0px";
  }

  set letterSpacing(value) {
    this.#brand;
    const prefix =
      "Failed to set 'letterSpacing' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get fontKerning() {
    this.#brand;
    return "auto";
  }

  set fontKerning(value) {
    this.#brand;
    const prefix =
      "Failed to set 'fontKerning' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasFontKerning(value);
    if (value === null) {
      return;
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get fontStretch() {
    this.#brand;
    return "normal";
  }

  set fontStretch(value) {
    this.#brand;
    const prefix =
      "Failed to set 'fontStretch' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasFontStretch(value);
    if (value === null) {
      return;
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get fontVariantCaps() {
    this.#brand;
    return "normal";
  }

  set fontVariantCaps(value) {
    this.#brand;
    const prefix =
      "Failed to set 'fontVariantCaps' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasFontVariantCaps(value);
    if (value === null) {
      return;
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get textRendering() {
    this.#brand;
    return "auto";
  }

  set textRendering(value) {
    this.#brand;
    const prefix =
      "Failed to set 'textRendering' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertCanvasTextRendering(value);
    if (value === null) {
      return;
    }
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  get wordSpacing() {
    this.#brand;
    return "0px";
  }

  set wordSpacing(value) {
    this.#brand;
    const prefix =
      "Failed to set 'wordSpacing' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    // TODO implement
    throw new TypeError("Unimplemented");
  }

  closePath() {
    this.#brand;
    op_canvas_2d_path_close(this.#defaultPath);
  }

  moveTo(x, y) {
    this.#brand;
    const prefix =
      "Failed to execute 'moveTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_move_to(this.#defaultPath, x, y);
  }

  lineTo(x, y) {
    this.#brand;
    const prefix =
      "Failed to execute 'lineTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_line_to(this.#defaultPath, x, y);
  }

  quadraticCurveTo(cpx, cpy, x, y) {
    this.#brand;
    const prefix =
      "Failed to execute 'quadraticCurveTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    cpx = convertUnrestrictedDouble(cpx);
    cpy = convertUnrestrictedDouble(cpy);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_quad_to(this.#defaultPath, cpx, cpy, x, y);
  }

  bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y) {
    this.#brand;
    const prefix =
      "Failed to execute 'bezierCurveTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 6, prefix);
    cp1x = convertUnrestrictedDouble(cp1x);
    cp1y = convertUnrestrictedDouble(cp1y);
    cp2x = convertUnrestrictedDouble(cp2x);
    cp2y = convertUnrestrictedDouble(cp2y);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_cubic_to(this.#defaultPath, cp1x, cp1y, cp2x, cp2y, x, y);
  }

  arcTo(x1, y1, x2, y2, radius) {
    this.#brand;
    const prefix =
      "Failed to execute 'arcTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 5, prefix);
    x1 = convertUnrestrictedDouble(x1);
    y1 = convertUnrestrictedDouble(y1);
    x2 = convertUnrestrictedDouble(x2);
    y2 = convertUnrestrictedDouble(y2);
    radius = convertUnrestrictedDouble(radius);
    if (!allFinite(x1, y1, x2, y2, radius)) {
      return;
    }
    if (radius < 0) {
      op_canvas_2d_path_ensure_subpath(this.#defaultPath, x1, y1);
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_arc_to(this.#defaultPath, x1, y1, x2, y2, radius);
  }

  rect(x, y, w, h) {
    this.#brand;
    const prefix =
      "Failed to execute 'rect' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    op_canvas_2d_path_rect(this.#defaultPath, x, y, w, h);
  }

  roundRect(x, y, w, h, radii = 0) {
    this.#brand;
    const prefix =
      "Failed to execute 'roundRect' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    radii = convertUnrestrictedDoubleOrDOMPointInitOrSequenceThereof(radii);
    const normalizedRadii = normalizeAndScaleRadii(x, y, w, h, radii);
    if (!normalizedRadii) {
      return;
    }
    const { upperLeft, upperRight, lowerRight, lowerLeft } = normalizedRadii;
    op_canvas_2d_path_round_rect(
      this.#defaultPath,
      x,
      y,
      w,
      h,
      upperLeft.x,
      upperLeft.y,
      upperRight.x,
      upperRight.y,
      lowerRight.x,
      lowerRight.y,
      lowerLeft.x,
      lowerLeft.y,
    );
  }

  arc(x, y, radius, startAngle, endAngle, counterclockwise = false) {
    this.#brand;
    const prefix =
      "Failed to execute 'arc' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 5, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    radius = convertUnrestrictedDouble(radius);
    startAngle = convertUnrestrictedDouble(startAngle);
    endAngle = convertUnrestrictedDouble(endAngle);
    counterclockwise = convertBoolean(counterclockwise);
    if (!allFinite(x, y, radius, startAngle, endAngle)) {
      return;
    }
    if (radius < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_ellipse(
      this.#defaultPath,
      x,
      y,
      radius,
      radius,
      startAngle,
      endAngle,
      0,
      counterclockwise ? -1 : 1,
    );
  }

  ellipse(
    x,
    y,
    radiusX,
    radiusY,
    rotation,
    startAngle,
    endAngle,
    counterclockwise = false,
  ) {
    this.#brand;
    const prefix =
      "Failed to execute 'ellipse' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 7, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    radiusX = convertUnrestrictedDouble(radiusX);
    radiusY = convertUnrestrictedDouble(radiusY);
    rotation = convertUnrestrictedDouble(rotation);
    startAngle = convertUnrestrictedDouble(startAngle);
    endAngle = convertUnrestrictedDouble(endAngle);
    counterclockwise = convertBoolean(counterclockwise);
    if (!allFinite(x, y, radiusX, radiusY, rotation, startAngle, endAngle)) {
      return;
    }
    if (radiusX < 0 || radiusY < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_ellipse(
      this.#defaultPath,
      x,
      y,
      radiusX,
      radiusY,
      startAngle,
      endAngle,
      rotation,
      counterclockwise ? -1 : 1,
    );
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: [
          "canvas",
          "globalAlpha",
          "globalCompositeOperation",
          "imageSmoothingEnabled",
          "imageSmoothingQuality",
          "strokeStyle",
          "fillStyle",
          "shadowOffsetX",
          "shadowOffsetY",
          "shadowBlur",
          "shadowColor",
          "filter",
          "lineWidth",
          "lineCap",
          "lineJoin",
          "miterLimit",
          "lineDashOffset",
          "font",
          "textAlign",
          "textBaseline",
          "direction",
          "letterSpacing",
          "fontKerning",
          "fontStretch",
          "fontVariantCaps",
          "textRendering",
          "wordSpacing",
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
    // deno-lint-ignore prefer-primordials
    objectIsOffscreenCanvasRenderingContext2D = (o) => #brand in o;
    getOffscreenCanvasRenderingContext2DState = (o) => o.#state;
    hideSourceText(this);
    const proto = ObjectGetOwnPropertyDescriptors(this.prototype);
    hideSourceText(proto.commit.value);
    hideSourceText(proto.canvas.get);
    hideSourceText(proto.save.value);
    hideSourceText(proto.restore.value);
    hideSourceText(proto.reset.value);
    hideSourceText(proto.isContextLost.value);
    hideSourceText(proto.scale.value);
    hideSourceText(proto.rotate.value);
    hideSourceText(proto.translate.value);
    hideSourceText(proto.transform.value);
    hideSourceText(proto.getTransform.value);
    hideSourceText(proto.setTransform.value);
    hideSourceText(proto.resetTransform.value);
    hideSourceText(proto.globalAlpha.get);
    hideSourceText(proto.globalAlpha.set);
    hideSourceText(proto.globalCompositeOperation.get);
    hideSourceText(proto.globalCompositeOperation.set);
    hideSourceText(proto.imageSmoothingEnabled.get);
    hideSourceText(proto.imageSmoothingEnabled.set);
    hideSourceText(proto.imageSmoothingQuality.get);
    hideSourceText(proto.imageSmoothingQuality.set);
    hideSourceText(proto.strokeStyle.get);
    hideSourceText(proto.strokeStyle.set);
    hideSourceText(proto.fillStyle.get);
    hideSourceText(proto.fillStyle.set);
    hideSourceText(proto.shadowOffsetX.get);
    hideSourceText(proto.shadowOffsetX.set);
    hideSourceText(proto.shadowOffsetY.get);
    hideSourceText(proto.shadowOffsetY.set);
    hideSourceText(proto.shadowBlur.get);
    hideSourceText(proto.shadowBlur.set);
    hideSourceText(proto.shadowColor.get);
    hideSourceText(proto.shadowColor.set);
    hideSourceText(proto.filter.get);
    hideSourceText(proto.filter.set);
    hideSourceText(proto.clearRect.value);
    hideSourceText(proto.fillRect.value);
    hideSourceText(proto.strokeRect.value);
    hideSourceText(proto.beginPath.value);
    hideSourceText(proto.fill.value);
    hideSourceText(proto.stroke.value);
    hideSourceText(proto.clip.value);
    hideSourceText(proto.isPointInPath.value);
    hideSourceText(proto.isPointInStroke.value);
    hideSourceText(proto.fillText.value);
    hideSourceText(proto.strokeText.value);
    hideSourceText(proto.measureText.value);
    hideSourceText(proto.drawImage.value);
    hideSourceText(proto.createImageData.value);
    hideSourceText(proto.getImageData.value);
    hideSourceText(proto.putImageData.value);
    hideSourceText(proto.lineWidth.get);
    hideSourceText(proto.lineWidth.set);
    hideSourceText(proto.lineCap.get);
    hideSourceText(proto.lineCap.set);
    hideSourceText(proto.lineJoin.get);
    hideSourceText(proto.lineJoin.set);
    hideSourceText(proto.miterLimit.get);
    hideSourceText(proto.miterLimit.set);
    hideSourceText(proto.setLineDash.value);
    hideSourceText(proto.getLineDash.value);
    hideSourceText(proto.lineDashOffset.get);
    hideSourceText(proto.lineDashOffset.set);
    hideSourceText(proto.font.get);
    hideSourceText(proto.font.set);
    hideSourceText(proto.textAlign.get);
    hideSourceText(proto.textAlign.set);
    hideSourceText(proto.textBaseline.get);
    hideSourceText(proto.textBaseline.set);
    hideSourceText(proto.direction.get);
    hideSourceText(proto.direction.set);
    hideSourceText(proto.letterSpacing.get);
    hideSourceText(proto.letterSpacing.set);
    hideSourceText(proto.fontKerning.get);
    hideSourceText(proto.fontKerning.set);
    hideSourceText(proto.fontStretch.get);
    hideSourceText(proto.fontStretch.set);
    hideSourceText(proto.fontVariantCaps.get);
    hideSourceText(proto.fontVariantCaps.set);
    hideSourceText(proto.textRendering.get);
    hideSourceText(proto.textRendering.set);
    hideSourceText(proto.wordSpacing.get);
    hideSourceText(proto.wordSpacing.set);
    hideSourceText(proto.closePath.value);
    hideSourceText(proto.moveTo.value);
    hideSourceText(proto.lineTo.value);
    hideSourceText(proto.quadraticCurveTo.value);
    hideSourceText(proto.bezierCurveTo.value);
    hideSourceText(proto.arcTo.value);
    hideSourceText(proto.rect.value);
    hideSourceText(proto.roundRect.value);
    hideSourceText(proto.arc.value);
    hideSourceText(proto.ellipse.value);
  }
}

let objectIsImageBitmap;
let getImageBitmapRaw;

export class ImageBitmap {
  #brand() {}

  #raw;

  constructor(key = undefined, raw) {
    if (key !== illegalConstructorKey) {
      illegalConstructor();
    }
    this.#raw = raw;
  }

  get width() {
    return this.#raw ? op_canvas_2d_image_bitmap_width(this.#raw) : 0;
  }

  get height() {
    return this.#raw ? op_canvas_2d_image_bitmap_height(this.#raw) : 0;
  }

  close() {
    if (this.#raw) {
      op_canvas_2d_image_bitmap_close(this.#raw);
      this.#raw = null;
    }
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["width", "height"],
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
    // deno-lint-ignore prefer-primordials
    objectIsImageBitmap = (o) => #brand in o;
    getImageBitmapRaw = (o) => o.#raw;
    hideSourceText(this);
    const proto = ObjectGetOwnPropertyDescriptors(this.prototype);
    hideSourceText(proto.width.get);
    hideSourceText(proto.height.get);
    hideSourceText(proto.close.value);
  }
}

const convertImageBitmapSource = (value) => {
  if (
    (type(value) === "Object" &&
      (objectIsImageBitmap(value) || objectIsOffscreenCanvas(value))) ||
    isBlob(value) || isImageData(value)
  ) {
    return value;
  }
  throw new TypeError("Expected ImageBitmapSource");
};
const convertImageOrientation = createEnumConverter(
  "ImageOrientation",
  ["from-image", "flipY"],
);
const convertPremultiplyAlpha = createEnumConverter(
  "PremultiplyAlpha",
  ["none", "premultiply", "default"],
);
const convertColorSpaceConversion = createEnumConverter(
  "ColorSpaceConversion",
  ["none", "default"],
);
const convertResizeQuality = createEnumConverter(
  "ResizeQuality",
  ["pixelated", "low", "medium", "high"],
);
const readImageBitmapOptionsMembers = (value) => {
  const result = { __proto__: null };
  const { imageOrientation = "from-image" } = value;
  result.imageOrientation = convertImageOrientation(imageOrientation);
  const { premultiplyAlpha = "default" } = value;
  result.premultiplyAlpha = convertPremultiplyAlpha(premultiplyAlpha);
  const { colorSpaceConversion = "default" } = value;
  result.colorSpaceConversion = convertColorSpaceConversion(
    colorSpaceConversion,
  );
  const { resizeWidth } = value;
  if (resizeWidth !== undefined) {
    result.resizeWidth = convertEnforceRangeUnsignedLong(resizeWidth);
  }
  const { resizeHeight } = value;
  if (resizeHeight !== undefined) {
    result.resizeHeight = convertEnforceRangeUnsignedLong(resizeHeight);
  }
  const { resizeQuality = "low" } = value;
  result.resizeQuality = convertResizeQuality(resizeQuality);
  return result;
};
const convertImageBitmapOptions = createDictionaryConverter(
  readImageBitmapOptionsMembers,
);
const imageOrientationToRepr = ObjectFreeze({
  __proto__: null,
  "from-image": 0,
  "flipY": 1,
});
const resizeQualityToRepr = ObjectFreeze({
  __proto__: null,
  "pixelated": 0,
  "low": 1,
  "medium": 2,
  "high": 3,
});
const checkUsabilityAndCropWithFormatting = (
  image,
  sx,
  sy,
  sw,
  sh,
  dw,
  dh,
  resizeQuality,
  imageOrientation,
) => {
  if (objectIsOffscreenCanvas(image)) {
    const ctx = getOffscreenCanvasContext(image);
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const width = getOffscreenCanvasWidth(image);
    const height = getOffscreenCanvasHeight(image);
    if (width === 0 || height === 0) {
      throw new DOMException("Canvas has no pixels", "InvalidStateError");
    }
    switch (getOffscreenCanvasContextMode(ctx)) {
      case "none":
        return op_canvas_2d_image_bitmap_empty_resize(
          sw ?? ctx.width,
          sh ?? ctx.height,
          dw ?? 0,
          dh ?? 0,
        );
      case "2d": {
        const state = getOffscreenCanvasRenderingContext2DState(ctx);
        return op_canvas_2d_image_bitmap_resize(
          op_canvas_2d_image_bitmap_from_canvas_state_cropped(
            state,
            sx,
            sy,
            sw ?? 0,
            sh ?? 0,
          ),
          dw ?? 0,
          dh ?? 0,
          resizeQualityToRepr[resizeQuality],
          imageOrientationToRepr[imageOrientation],
        );
      }
    }
  }
  if (objectIsImageBitmap(image)) {
    const raw = getImageBitmapRaw(image);
    if (raw === null) {
      throw new DOMException("Image is detached", "InvalidStateError");
    }
    return op_canvas_2d_image_bitmap_resize(
      op_canvas_2d_image_bitmap_crop(raw, sx, sy, sw ?? 0, sh ?? 0),
      dw ?? 0,
      dh ?? 0,
      resizeQualityToRepr[resizeQuality],
      imageOrientationToRepr[imageOrientation],
    );
  }
  if (isBlob(image)) {
    throw new DOMException("Unsupported image format", "InvalidStateError");
  }
  const data = ImageDataPrototypeGetData(image);
  if (TypedArrayPrototypeGetLength(data) === 0) {
    throw new DOMException("Image data is detached", "InvalidStateError");
  }
  const buf = alignUint8ClampedArrayToUint32(data);
  const width = ImageDataPrototypeGetWidth(image);
  const height = ImageDataPrototypeGetHeight(image);
  const colorSpace = ImageDataPrototypeGetColorSpace(image);
  return op_canvas_2d_image_bitmap_resize(
    op_canvas_2d_image_bitmap_from_image_data_cropped(
      buf,
      width,
      height,
      colorSpaceToRepr[colorSpace],
      sx,
      sy,
      sw ?? 0,
      sh ?? 0,
    ),
    dw ?? 0,
    dh ?? 0,
    resizeQualityToRepr[resizeQuality],
    imageOrientationToRepr[imageOrientation],
  );
};

export const makeCreateImageBitmap = (prefix) =>
  // deno-lint-ignore require-await
  hideSourceText(async function createImageBitmap(
    image,
    sx = undefined,
    sy,
    sw,
    sh,
    options,
  ) {
    if (this !== null && this !== undefined && this !== globalThis) {
      throw new TypeError("Illegal invocation");
    }
    const nArgs = arguments.length;
    requiredArguments(nArgs, 1, prefix);
    if (nArgs <= 2) {
      image = convertImageBitmapSource(image);
      options = convertImageBitmapOptions(sx);
      sx = 0;
      sy = 0;
    } else if (nArgs >= 5) {
      image = convertImageBitmapSource(image);
      sx = convertLong(sx);
      sy = convertLong(sy);
      sw = convertLong(sw);
      sh = convertLong(sh);
      options = convertImageBitmapOptions(options);
      if (sw === 0) {
        throw new RangeError("Source width must be non-zero");
      }
      if (sh === 0) {
        throw new RangeError("Source height must be non-zero");
      }
      if (sw < 0) {
        sx += sw;
        sw = -sw;
      }
      if (sh < 0) {
        sy += sh;
        sh = -sh;
      }
    } else {
      throw new TypeError("Overload resolution failed");
    }
    const {
      resizeWidth,
      resizeHeight,
      resizeQuality,
      imageOrientation,
    } = options;
    if (resizeWidth === 0) {
      throw new DOMException(
        "Output width must be non-zero",
        "InvalidStateError",
      );
    }
    if (resizeHeight === 0) {
      throw new DOMException(
        "Output height must be non-zero",
        "InvalidStateError",
      );
    }
    const bitmap = checkUsabilityAndCropWithFormatting(
      image,
      sx,
      sy,
      sw,
      sh,
      resizeWidth,
      resizeHeight,
      resizeQuality,
      imageOrientation,
    );
    return new ImageBitmap(illegalConstructorKey, bitmap);
  });
