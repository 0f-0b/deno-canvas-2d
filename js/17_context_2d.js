import { primordials } from "ext:core/mod.js";
import {
  ImageDataPrototypeGetColorSpace,
  ImageDataPrototypeGetData,
  ImageDataPrototypeGetHeight,
  ImageDataPrototypeGetWidth,
} from "ext:deno_canvas_2d/00_image_data_primordials.js";
import {
  op_canvas_2d_gradient_add_color_stop,
  op_canvas_2d_gradient_new_conic,
  op_canvas_2d_gradient_new_linear,
  op_canvas_2d_gradient_new_radial,
  op_canvas_2d_image_bitmap_from_canvas_state,
  op_canvas_2d_image_bitmap_from_canvas_state_cropped,
  op_canvas_2d_image_bitmap_height,
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
  op_canvas_2d_state_direction,
  op_canvas_2d_state_draw_image,
  op_canvas_2d_state_fill,
  op_canvas_2d_state_fill_rect,
  op_canvas_2d_state_fill_style,
  op_canvas_2d_state_fill_text,
  op_canvas_2d_state_font,
  op_canvas_2d_state_font_kerning,
  op_canvas_2d_state_font_stretch,
  op_canvas_2d_state_font_variant_caps,
  op_canvas_2d_state_get_image_data,
  op_canvas_2d_state_get_transform,
  op_canvas_2d_state_global_alpha,
  op_canvas_2d_state_global_composite_operation,
  op_canvas_2d_state_height,
  op_canvas_2d_state_image_smoothing_enabled,
  op_canvas_2d_state_image_smoothing_quality,
  op_canvas_2d_state_is_point_in_path,
  op_canvas_2d_state_is_point_in_stroke,
  op_canvas_2d_state_letter_spacing,
  op_canvas_2d_state_line_cap,
  op_canvas_2d_state_line_dash_offset,
  op_canvas_2d_state_line_join,
  op_canvas_2d_state_line_width,
  op_canvas_2d_state_measure_text,
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
  op_canvas_2d_state_set_direction,
  op_canvas_2d_state_set_fill_style_color,
  op_canvas_2d_state_set_fill_style_gradient,
  op_canvas_2d_state_set_fill_style_pattern,
  op_canvas_2d_state_set_filter,
  op_canvas_2d_state_set_font,
  op_canvas_2d_state_set_font_kerning,
  op_canvas_2d_state_set_font_stretch,
  op_canvas_2d_state_set_font_variant_caps,
  op_canvas_2d_state_set_global_alpha,
  op_canvas_2d_state_set_global_composite_operation,
  op_canvas_2d_state_set_height,
  op_canvas_2d_state_set_image_smoothing_enabled,
  op_canvas_2d_state_set_image_smoothing_quality,
  op_canvas_2d_state_set_letter_spacing,
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
  op_canvas_2d_state_set_text_align,
  op_canvas_2d_state_set_text_baseline,
  op_canvas_2d_state_set_text_rendering,
  op_canvas_2d_state_set_transform,
  op_canvas_2d_state_set_width,
  op_canvas_2d_state_set_word_spacing,
  op_canvas_2d_state_shadow_blur,
  op_canvas_2d_state_shadow_color,
  op_canvas_2d_state_shadow_offset_x,
  op_canvas_2d_state_shadow_offset_y,
  op_canvas_2d_state_stroke,
  op_canvas_2d_state_stroke_rect,
  op_canvas_2d_state_stroke_style,
  op_canvas_2d_state_stroke_text,
  op_canvas_2d_state_text_align,
  op_canvas_2d_state_text_baseline,
  op_canvas_2d_state_text_rendering,
  op_canvas_2d_state_transform,
  op_canvas_2d_state_translate,
  op_canvas_2d_state_width,
  op_canvas_2d_state_word_spacing,
} from "ext:deno_canvas_2d/00_ops.js";
import { defaultTo } from "ext:deno_canvas_2d/01_default_to.js";
import { requireObject } from "ext:deno_canvas_2d/01_require_object.js";
import { createDictionaryConverter } from "ext:deno_canvas_2d/04_create_dictionary_converter.js";
import { createEnumConverter } from "ext:deno_canvas_2d/04_create_enum_converter.js";
import { createSequenceFromIterable } from "ext:deno_canvas_2d/04_create_sequence_from_iterable.js";
import { convertBoolean } from "ext:deno_canvas_2d/05_convert_boolean.js";
import { convertDOMString } from "ext:deno_canvas_2d/05_convert_dom_string.js";
import { convertDouble } from "ext:deno_canvas_2d/05_convert_double.js";
import { convertEnforceRangeLong } from "ext:deno_canvas_2d/05_convert_enforce_range_long.js";
import { convertImageData } from "ext:deno_canvas_2d/05_convert_image_data.js";
import { convertLegacyNullToEmptyStringDOMString } from "ext:deno_canvas_2d/05_convert_legacy_null_to_empty_string_dom_string.js";
import { convertUnrestrictedDouble } from "ext:deno_canvas_2d/05_convert_unrestricted_double.js";
import {
  convertDOMMatrix2DInit,
  convertDOMPointInit,
  directConstruct,
  DOMMatrix,
  validateAndFixup2D,
} from "ext:deno_canvas_2d/15_geometry.js";
import {
  alignUint8ClampedArrayToUint32,
  checkUsabilityAndClone,
  colorSpaceToRepr,
  ImageBitmap,
  objectIsImageBitmap,
  objectIsOffscreenCanvas,
  registerCanvasContextMode,
} from "ext:deno_canvas_2d/16_canvas.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
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
  RangeError,
  SymbolFor,
  SymbolIterator,
  TypeError,
  TypedArrayPrototypeGetBuffer,
  TypedArrayPrototypeGetLength,
  Uint32Array,
  Uint8Array,
} = primordials;
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
const convertDOMStringOrCanvasGradientOrCanvasPattern = (value) => {
  if (
    type(value) === "Object" &&
    (objectIsCanvasGradient(value) || objectIsCanvasPattern(value))
  ) {
    return value;
  }
  return convertDOMString(value);
};
let objectIsCanvasGradient;
let getCanvasGradientRaw;

export class CanvasGradient {
  #brand() {}

  #raw;

  constructor(key = undefined, raw) {
    if (key !== illegalConstructor) {
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
  }
}

const linearGradient = (x0, y0, x1, y1) =>
  new CanvasGradient(
    illegalConstructor,
    op_canvas_2d_gradient_new_linear(x0, y0, x1, y1),
  );
const radialGradient = (x0, y0, r0, x1, y1, r1) =>
  new CanvasGradient(
    illegalConstructor,
    op_canvas_2d_gradient_new_radial(x0, y0, r0, x1, y1, r1),
  );
const conicGradient = (startAngle, x, y) =>
  new CanvasGradient(
    illegalConstructor,
    op_canvas_2d_gradient_new_conic(startAngle, x, y),
  );
let objectIsCanvasPattern;
let getCanvasPatternRaw;

export class CanvasPattern {
  #brand() {}

  #raw;

  constructor(key = undefined, raw) {
    if (key !== illegalConstructor) {
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
  }
}

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
    illegalConstructor,
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
    if (key !== illegalConstructor) {
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
  }
}

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
let objectIsOffscreenCanvasRenderingContext2D;
let getOffscreenCanvasRenderingContext2DState;
let getOffscreenCanvasRenderingContext2DColorSpace;
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
const textAlignFromRepr = ObjectFreeze([
  "start",
  "end",
  "left",
  "right",
  "center",
]);
const textAlignToRepr = ObjectFreeze({
  __proto__: null,
  "start": 0,
  "end": 1,
  "left": 2,
  "right": 3,
  "center": 4,
});
const textBaselineFromRepr = ObjectFreeze([
  "top",
  "hanging",
  "middle",
  "alphabetic",
  "ideographic",
  "bottom",
]);
const textBaselineToRepr = ObjectFreeze({
  __proto__: null,
  "top": 0,
  "hanging": 1,
  "middle": 2,
  "alphabetic": 3,
  "ideographic": 4,
  "bottom": 5,
});
const directionFromRepr = ObjectFreeze([
  "ltr",
  "rtl",
  "inherit",
]);
const directionToRepr = ObjectFreeze({
  __proto__: null,
  "ltr": 0,
  "rtl": 1,
  "inherit": 2,
});
const fontKerningFromRepr = ObjectFreeze([
  "auto",
  "normal",
  "none",
]);
const fontKerningToRepr = ObjectFreeze({
  __proto__: null,
  "auto": 0,
  "normal": 1,
  "none": 2,
});
const fontStretchFromRepr = ObjectFreeze([
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
const fontStretchToRepr = ObjectFreeze({
  __proto__: null,
  "ultra-condensed": 0,
  "extra-condensed": 1,
  "condensed": 2,
  "semi-condensed": 3,
  "normal": 4,
  "semi-expanded": 5,
  "expanded": 6,
  "extra-expanded": 7,
  "ultra-expanded": 8,
});
const fontVariantCapsFromRepr = ObjectFreeze([
  "normal",
  "small-caps",
  "all-small-caps",
  "petite-caps",
  "all-petite-caps",
  "unicase",
  "titling-caps",
]);
const fontVariantCapsToRepr = ObjectFreeze({
  __proto__: null,
  "normal": 0,
  "small-caps": 1,
  "all-small-caps": 2,
  "petite-caps": 3,
  "all-petite-caps": 4,
  "unicase": 5,
  "titling-caps": 6,
});
const textRenderingFromRepr = ObjectFreeze([
  "auto",
  "optimizeSpeed",
  "optimizeLegibility",
  "geometricPrecision",
]);
const textRenderingToRepr = ObjectFreeze({
  __proto__: null,
  "auto": 0,
  "optimizeSpeed": 1,
  "optimizeLegibility": 2,
  "geometricPrecision": 3,
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
const measureTextBuffer = new Float64Array(12);

export class OffscreenCanvasRenderingContext2D {
  #brand() {}

  #canvas;
  #state;
  #colorSpace;
  #cachedFont = "10px sans-serif";
  #cachedLetterSpacing = "0px";
  #cachedWordSpacing = "0px";
  #cachedFillStyle = null;
  #cachedStrokeStyle = null;
  #cachedDefaultPath = null;
  #cachedShadowColor = null;
  #cachedFilter = "none";

  constructor(key = undefined, target, width, height, settings) {
    if (key !== illegalConstructor) {
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
    value = convertDOMString(value);
    const repr = imageSmoothingQualityToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_image_smoothing_quality(this.#state, repr);
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
      if (op_canvas_2d_state_set_stroke_style_color(this.#state, value)) {
        this.#cachedStrokeStyle = null;
      }
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
      if (op_canvas_2d_state_set_fill_style_color(this.#state, value)) {
        this.#cachedFillStyle = null;
      }
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
    if (op_canvas_2d_state_set_shadow_color(this.#state, value)) {
      this.#cachedShadowColor = null;
    }
  }

  get filter() {
    this.#brand;
    return this.#cachedFilter;
  }

  set filter(value) {
    this.#brand;
    const prefix =
      "Failed to set 'filter' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    if (op_canvas_2d_state_set_filter(this.#state, value)) {
      this.#cachedFilter = value;
    }
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
    if (maxWidth === undefined) {
      maxWidth = Infinity;
    } else {
      maxWidth = convertUnrestrictedDouble(maxWidth);
      if (!NumberIsFinite(maxWidth)) {
        return;
      }
    }
    op_canvas_2d_state_fill_text(this.#state, text, x, y, maxWidth);
  }

  strokeText(text, x, y, maxWidth = undefined) {
    this.#brand;
    const prefix =
      "Failed to execute 'strokeText' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 3, prefix);
    text = convertDOMString(text);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    if (maxWidth === undefined) {
      maxWidth = Infinity;
    } else {
      maxWidth = convertUnrestrictedDouble(maxWidth);
      if (!NumberIsFinite(maxWidth)) {
        return;
      }
    }
    op_canvas_2d_state_stroke_text(this.#state, text, x, y, maxWidth);
  }

  measureText(text) {
    this.#brand;
    const prefix =
      "Failed to execute 'measureText' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    text = convertDOMString(text);
    op_canvas_2d_state_measure_text(this.#state, text, measureTextBuffer);
    return new TextMetrics(
      illegalConstructor,
      measureTextBuffer[0],
      measureTextBuffer[1],
      measureTextBuffer[2],
      measureTextBuffer[3],
      measureTextBuffer[4],
      measureTextBuffer[5],
      measureTextBuffer[6],
      measureTextBuffer[7],
      measureTextBuffer[8],
      measureTextBuffer[9],
      measureTextBuffer[10],
      measureTextBuffer[11],
    );
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
    value = convertDOMString(value);
    const repr = lineCapToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_line_cap(this.#state, repr);
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
    value = convertDOMString(value);
    const repr = lineJoinToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_line_join(this.#state, repr);
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
    this.#cachedFont ??= op_canvas_2d_state_font(this.#state);
    return this.#cachedFont;
  }

  set font(value) {
    this.#brand;
    const prefix =
      "Failed to set 'font' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    if (op_canvas_2d_state_set_font(this.#state, value)) {
      this.#cachedFont = null;
    }
  }

  get textAlign() {
    this.#brand;
    return textAlignFromRepr[op_canvas_2d_state_text_align(this.#state)];
  }

  set textAlign(value) {
    this.#brand;
    const prefix =
      "Failed to set 'textAlign' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = textAlignToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_text_align(this.#state, repr);
  }

  get textBaseline() {
    this.#brand;
    return textBaselineFromRepr[op_canvas_2d_state_text_baseline(this.#state)];
  }

  set textBaseline(value) {
    this.#brand;
    const prefix =
      "Failed to set 'textBaseline' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = textBaselineToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_text_baseline(this.#state, repr);
  }

  get direction() {
    this.#brand;
    return directionFromRepr[op_canvas_2d_state_direction(this.#state)];
  }

  set direction(value) {
    this.#brand;
    const prefix =
      "Failed to set 'direction' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = directionToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_direction(this.#state, repr);
  }

  get letterSpacing() {
    this.#brand;
    this.#cachedLetterSpacing ??= op_canvas_2d_state_letter_spacing(
      this.#state,
    );
    return this.#cachedLetterSpacing;
  }

  set letterSpacing(value) {
    this.#brand;
    const prefix =
      "Failed to set 'letterSpacing' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    if (op_canvas_2d_state_set_letter_spacing(this.#state, value)) {
      this.#cachedLetterSpacing = null;
    }
  }

  get fontKerning() {
    this.#brand;
    return fontKerningFromRepr[op_canvas_2d_state_font_kerning(this.#state)];
  }

  set fontKerning(value) {
    this.#brand;
    const prefix =
      "Failed to set 'fontKerning' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = fontKerningToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_font_kerning(this.#state, repr);
  }

  get fontStretch() {
    this.#brand;
    return fontStretchFromRepr[op_canvas_2d_state_font_stretch(this.#state)];
  }

  set fontStretch(value) {
    this.#brand;
    const prefix =
      "Failed to set 'fontStretch' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = fontStretchToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_font_stretch(this.#state, repr);
  }

  get fontVariantCaps() {
    this.#brand;
    return fontVariantCapsFromRepr[
      op_canvas_2d_state_font_variant_caps(this.#state)
    ];
  }

  set fontVariantCaps(value) {
    this.#brand;
    const prefix =
      "Failed to set 'fontVariantCaps' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = fontVariantCapsToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_font_variant_caps(this.#state, repr);
  }

  get textRendering() {
    this.#brand;
    return textRenderingFromRepr[
      op_canvas_2d_state_text_rendering(this.#state)
    ];
  }

  set textRendering(value) {
    this.#brand;
    const prefix =
      "Failed to set 'textRendering' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = textRenderingToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_text_rendering(this.#state, repr);
  }

  get wordSpacing() {
    this.#brand;
    this.#cachedWordSpacing ??= op_canvas_2d_state_word_spacing(this.#state);
    return this.#cachedWordSpacing;
  }

  set wordSpacing(value) {
    this.#brand;
    const prefix =
      "Failed to set 'wordSpacing' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    if (op_canvas_2d_state_set_word_spacing(this.#state, value)) {
      this.#cachedWordSpacing = null;
    }
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
    getOffscreenCanvasRenderingContext2DColorSpace = (o) => o.#colorSpace;
  }
}

registerCanvasContextMode("2d", {
  newInstance(canvas, width, height, options) {
    return new OffscreenCanvasRenderingContext2D(
      illegalConstructor,
      canvas,
      width,
      height,
      options,
    );
  },
  hasInstance(ctx) {
    return objectIsOffscreenCanvasRenderingContext2D(ctx);
  },
  getWidth(ctx) {
    const state = getOffscreenCanvasRenderingContext2DState(ctx);
    return op_canvas_2d_state_width(state);
  },
  setWidth(ctx, value) {
    const state = getOffscreenCanvasRenderingContext2DState(ctx);
    op_canvas_2d_state_set_width(state, value);
  },
  getHeight(ctx) {
    const state = getOffscreenCanvasRenderingContext2DState(ctx);
    return op_canvas_2d_state_height(state);
  },
  setHeight(ctx, value) {
    const state = getOffscreenCanvasRenderingContext2DState(ctx);
    op_canvas_2d_state_set_height(state, value);
  },
  transferToImageBitmap(ctx) {
    const state = getOffscreenCanvasRenderingContext2DState(ctx);
    const bitmap = op_canvas_2d_image_bitmap_from_canvas_state(state);
    op_canvas_2d_state_clear(state);
    return new ImageBitmap(illegalConstructor, bitmap);
  },
  cloneToImageBitmap(ctx) {
    const state = getOffscreenCanvasRenderingContext2DState(ctx);
    return op_canvas_2d_image_bitmap_from_canvas_state(state);
  },
  cropToImageBitmap(ctx, sx, sy, sw, sh) {
    const state = getOffscreenCanvasRenderingContext2DState(ctx);
    return {
      bitmap: op_canvas_2d_image_bitmap_from_canvas_state_cropped(
        state,
        sx,
        sy,
        sw ?? 0,
        sh ?? 0,
      ),
      needResize: true,
    };
  },
  getDataForSerialization(ctx, colorSpace) {
    colorSpace ??= getOffscreenCanvasRenderingContext2DColorSpace(ctx);
    const state = getOffscreenCanvasRenderingContext2DState(ctx);
    const width = op_canvas_2d_state_width(state);
    const height = op_canvas_2d_state_height(state);
    const buf = new Uint32Array(width * height);
    op_canvas_2d_state_get_image_data(
      state,
      buf,
      width,
      height,
      colorSpaceToRepr[colorSpace],
      0,
      0,
    );
    const data = new Uint8Array(TypedArrayPrototypeGetBuffer(buf));
    return { data, width, height, colorSpace };
  },
});
