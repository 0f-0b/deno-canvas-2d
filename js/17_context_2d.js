import { primordials } from "ext:core/mod.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import { ImageData } from "ext:deno_web/16_image_data.js";
import {
  configureInterface,
  illegalConstructor,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";
import {
  ImageDataPrototypeGetColorSpace,
  ImageDataPrototypeGetData,
  ImageDataPrototypeGetHeight,
  ImageDataPrototypeGetWidth,
} from "./00_image_data_primordials.js";
import {
  IntlLocale,
  IntlLocalePrototypeGetBaseName,
  IntlLocalePrototypeGetScript,
  IntlLocalePrototypeMaximize,
} from "./00_intl_locale_primordials.js";
import {
  op_canvas_2d_gradient_add_color_stop,
  op_canvas_2d_gradient_new_conic,
  op_canvas_2d_gradient_new_linear,
  op_canvas_2d_gradient_new_radial,
  op_canvas_2d_image_bitmap_from_canvas_state,
  op_canvas_2d_image_bitmap_from_canvas_state_crop,
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
  op_canvas_2d_path_from_svg,
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
  op_canvas_2d_state_set_lang,
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
} from "./00_ops.js";
import { capturePrototype } from "./01_capture_prototype.js";
import { defaultTo } from "./01_default_to.js";
import { IdentityConstructor } from "./01_identity_constructor.js";
import { requireObject } from "./01_require_object.js";
import { createDictionaryConverter } from "./04_create_dictionary_converter.js";
import { createEnumConverter } from "./04_create_enum_converter.js";
import { createSequenceFromIterable } from "./04_create_sequence_from_iterable.js";
import { convertBoolean } from "./05_convert_boolean.js";
import { convertDOMString } from "./05_convert_dom_string.js";
import { convertDouble } from "./05_convert_double.js";
import { convertEnforceRangeLong } from "./05_convert_enforce_range_long.js";
import { convertImageData } from "./05_convert_image_data.js";
import { convertLegacyNullToEmptyStringDOMString } from "./05_convert_legacy_null_to_empty_string_dom_string.js";
import { convertUnrestrictedDouble } from "./05_convert_unrestricted_double.js";
import {
  convertDOMMatrix2DInit,
  convertDOMPointInit,
  createDOMMatrix,
  validateAndFixup2D,
} from "./15_geometry.js";
import {
  alignUint8ClampedArrayToUint32,
  checkUsabilityAndClone,
  colorSpaceToRepr,
  ImageBitmapInternals,
  OffscreenCanvasInternals,
  registerCanvasContextMode,
} from "./16_canvas.js";

const {
  ArrayIsArray,
  ArrayPrototypePop,
  ArrayPrototypePush,
  Float64Array,
  FunctionPrototype,
  MathAbs,
  MathMin,
  MathSign,
  NumberIsFinite,
  Object,
  ObjectCreate,
  ObjectFreeze,
  ObjectSetPrototypeOf,
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
    (ImageBitmapInternals.hasInstance(value) ||
      OffscreenCanvasInternals.hasInstance(value))
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
    (CanvasGradientInternals.hasInstance(value) ||
      CanvasPatternInternals.hasInstance(value))
  ) {
    return value;
  }
  return convertDOMString(value);
};
const CanvasGradientInternals = class CanvasGradient
  extends IdentityConstructor {
  #brand() {}

  #raw;

  constructor(o, raw) {
    super(o);
    this.#raw = raw;
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getRaw(o) {
    return o.#raw;
  }
};

export class CanvasGradient extends Object {
  // deno-lint-ignore constructor-super
  constructor() {
    illegalConstructor();
  }

  addColorStop(offset, color) {
    CanvasGradientInternals.checkInstance(this);
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
    op_canvas_2d_gradient_add_color_stop(
      CanvasGradientInternals.getRaw(this),
      offset,
      color,
    );
  }

  static {
    ObjectSetPrototypeOf(this, FunctionPrototype);
    configureInterface(this);
  }
}

function createCanvasGradientFromRaw(raw) {
  const o = ObjectCreate(CanvasGradient.prototype);
  new CanvasGradientInternals(o, raw);
  return o;
}

const linearGradient = (x0, y0, x1, y1) =>
  createCanvasGradientFromRaw(op_canvas_2d_gradient_new_linear(x0, y0, x1, y1));
const radialGradient = (x0, y0, r0, x1, y1, r1) =>
  createCanvasGradientFromRaw(
    op_canvas_2d_gradient_new_radial(x0, y0, r0, x1, y1, r1),
  );
const conicGradient = (startAngle, x, y) =>
  createCanvasGradientFromRaw(
    op_canvas_2d_gradient_new_conic(startAngle, x, y),
  );
const CanvasPatternInternals = class CanvasPattern extends IdentityConstructor {
  #brand() {}

  #raw;

  constructor(o, raw) {
    super(o);
    this.#raw = raw;
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getRaw(o) {
    return o.#raw;
  }
};

export class CanvasPattern extends Object {
  // deno-lint-ignore constructor-super
  constructor() {
    illegalConstructor();
  }

  setTransform(transform = undefined) {
    CanvasPatternInternals.checkInstance(this);
    transform = convertDOMMatrix2DInit(transform);
    validateAndFixup2D(transform);
    op_canvas_2d_pattern_set_transform(
      CanvasPatternInternals.getRaw(this),
      transform.m11,
      transform.m12,
      transform.m21,
      transform.m22,
      transform.m41,
      transform.m42,
    );
  }

  static {
    ObjectSetPrototypeOf(this, FunctionPrototype);
    configureInterface(this);
  }
}

function createCanvasPatternFromRaw(raw) {
  const o = ObjectCreate(CanvasPattern.prototype);
  new CanvasPatternInternals(o, raw);
  return o;
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
  return createCanvasPatternFromRaw(
    op_canvas_2d_pattern_new(bitmap, repetition),
  );
};
const TextMetricsInternals = class TextMetrics extends IdentityConstructor {
  #brand() {}

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

  constructor(o, values) {
    super(o);
    this.#width = values[0];
    this.#actualBoundingBoxLeft = values[1];
    this.#actualBoundingBoxRight = values[2];
    this.#fontBoundingBoxAscent = values[3];
    this.#fontBoundingBoxDescent = values[4];
    this.#actualBoundingBoxAscent = values[5];
    this.#actualBoundingBoxDescent = values[6];
    this.#emHeightAscent = values[7];
    this.#emHeightDescent = values[8];
    this.#hangingBaseline = values[9];
    this.#alphabeticBaseline = values[10];
    this.#ideographicBaseline = values[11];
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getWidth(o) {
    return o.#width;
  }

  static getActualBoundingBoxLeft(o) {
    return o.#actualBoundingBoxLeft;
  }

  static getActualBoundingBoxRight(o) {
    return o.#actualBoundingBoxRight;
  }

  static getFontBoundingBoxAscent(o) {
    return o.#fontBoundingBoxAscent;
  }

  static getFontBoundingBoxDescent(o) {
    return o.#fontBoundingBoxDescent;
  }

  static getActualBoundingBoxAscent(o) {
    return o.#actualBoundingBoxAscent;
  }

  static getActualBoundingBoxDescent(o) {
    return o.#actualBoundingBoxDescent;
  }

  static getEmHeightAscent(o) {
    return o.#emHeightAscent;
  }

  static getEmHeightDescent(o) {
    return o.#emHeightDescent;
  }

  static getHangingBaseline(o) {
    return o.#hangingBaseline;
  }

  static getAlphabeticBaseline(o) {
    return o.#alphabeticBaseline;
  }

  static getIdeographicBaseline(o) {
    return o.#ideographicBaseline;
  }

  static inspect(inspect, options) {
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
};

export class TextMetrics extends Object {
  // deno-lint-ignore constructor-super
  constructor() {
    illegalConstructor();
  }

  get width() {
    return TextMetricsInternals.getWidth(this);
  }

  get actualBoundingBoxLeft() {
    return TextMetricsInternals.getActualBoundingBoxLeft(this);
  }

  get actualBoundingBoxRight() {
    return TextMetricsInternals.getActualBoundingBoxRight(this);
  }

  get fontBoundingBoxAscent() {
    return TextMetricsInternals.getFontBoundingBoxAscent(this);
  }

  get fontBoundingBoxDescent() {
    return TextMetricsInternals.getFontBoundingBoxDescent(this);
  }

  get actualBoundingBoxAscent() {
    return TextMetricsInternals.getActualBoundingBoxAscent(this);
  }

  get actualBoundingBoxDescent() {
    return TextMetricsInternals.getActualBoundingBoxDescent(this);
  }

  get emHeightAscent() {
    return TextMetricsInternals.getEmHeightAscent(this);
  }

  get emHeightDescent() {
    return TextMetricsInternals.getEmHeightDescent(this);
  }

  get hangingBaseline() {
    return TextMetricsInternals.getHangingBaseline(this);
  }

  get alphabeticBaseline() {
    return TextMetricsInternals.getAlphabeticBaseline(this);
  }

  get ideographicBaseline() {
    return TextMetricsInternals.getIdeographicBaseline(this);
  }

  get [privateCustomInspect]() {
    return TextMetricsInternals.hasInstance(this)
      ? TextMetricsInternals.inspect
      : undefined;
  }

  static {
    ObjectSetPrototypeOf(this, FunctionPrototype);
    configureInterface(this);
  }
}

function createTextMetrics(values) {
  const o = ObjectCreate(TextMetrics.prototype);
  new TextMetricsInternals(o, values);
  return o;
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
  if (!(type(value) === "Object" && Path2DInternals.hasInstance(value))) {
    throw new TypeError("Expected Path2D");
  }
  return value;
};
const convertPath2DOrDOMString = (value) =>
  type(value) === "Object" && Path2DInternals.hasInstance(value)
    ? value
    : convertDOMString(value);

function normalizeAndScaleRadii(x, y, w, h, radii) {
  if (
    !(NumberIsFinite(x) && NumberIsFinite(y) &&
      NumberIsFinite(w) && NumberIsFinite(h))
  ) {
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
      if (!(NumberIsFinite(x) && NumberIsFinite(y))) {
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

const Path2DInternals = class Path2D extends IdentityConstructor {
  #brand() {}

  #raw;

  constructor(o, raw) {
    super(o);
    this.#raw = raw;
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getRaw(o) {
    return o.#raw;
  }
};

export class Path2D extends Object {
  constructor(path = undefined) {
    if (path !== undefined) {
      path = convertPath2DOrDOMString(path);
    }
    const newTarget = capturePrototype(new.target, Path2D);
    const o = ObjectCreate(newTarget.prototype);
    new Path2DInternals(
      o,
      path === undefined
        ? op_canvas_2d_path_new()
        : typeof path === "string"
        ? op_canvas_2d_path_from_svg(path)
        : op_canvas_2d_path_clone(Path2DInternals.getRaw(path)),
    );
    return o;
  }

  addPath(path, transform = undefined) {
    Path2DInternals.checkInstance(this);
    const prefix = "Failed to execute 'addPath' on 'Path2D'";
    requiredArguments(arguments.length, 1, prefix);
    path = convertPath2D(path);
    transform = convertDOMMatrix2DInit(transform);
    validateAndFixup2D(transform);
    op_canvas_2d_path_extend(
      Path2DInternals.getRaw(this),
      Path2DInternals.getRaw(path),
      transform.m11,
      transform.m12,
      transform.m21,
      transform.m22,
      transform.m41,
      transform.m42,
    );
  }

  closePath() {
    Path2DInternals.checkInstance(this);
    op_canvas_2d_path_close(Path2DInternals.getRaw(this));
  }

  moveTo(x, y) {
    Path2DInternals.checkInstance(this);
    const prefix = "Failed to execute 'moveTo' on 'Path2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_move_to(Path2DInternals.getRaw(this), x, y);
  }

  lineTo(x, y) {
    Path2DInternals.checkInstance(this);
    const prefix = "Failed to execute 'lineTo' on 'Path2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_line_to(Path2DInternals.getRaw(this), x, y);
  }

  quadraticCurveTo(cpx, cpy, x, y) {
    Path2DInternals.checkInstance(this);
    const prefix = "Failed to execute 'quadraticCurveTo' on 'Path2D'";
    requiredArguments(arguments.length, 4, prefix);
    cpx = convertUnrestrictedDouble(cpx);
    cpy = convertUnrestrictedDouble(cpy);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_quad_to(Path2DInternals.getRaw(this), cpx, cpy, x, y);
  }

  bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y) {
    Path2DInternals.checkInstance(this);
    const prefix = "Failed to execute 'bezierCurveTo' on 'Path2D'";
    requiredArguments(arguments.length, 6, prefix);
    cp1x = convertUnrestrictedDouble(cp1x);
    cp1y = convertUnrestrictedDouble(cp1y);
    cp2x = convertUnrestrictedDouble(cp2x);
    cp2y = convertUnrestrictedDouble(cp2y);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_cubic_to(
      Path2DInternals.getRaw(this),
      cp1x,
      cp1y,
      cp2x,
      cp2y,
      x,
      y,
    );
  }

  arcTo(x1, y1, x2, y2, radius) {
    Path2DInternals.checkInstance(this);
    const prefix = "Failed to execute 'arcTo' on 'Path2D'";
    requiredArguments(arguments.length, 5, prefix);
    x1 = convertUnrestrictedDouble(x1);
    y1 = convertUnrestrictedDouble(y1);
    x2 = convertUnrestrictedDouble(x2);
    y2 = convertUnrestrictedDouble(y2);
    radius = convertUnrestrictedDouble(radius);
    if (
      !(NumberIsFinite(x1) && NumberIsFinite(y1) &&
        NumberIsFinite(x2) && NumberIsFinite(y2) && NumberIsFinite(radius))
    ) {
      return;
    }
    if (radius < 0) {
      op_canvas_2d_path_ensure_subpath(Path2DInternals.getRaw(this), x1, y1);
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_arc_to(
      Path2DInternals.getRaw(this),
      x1,
      y1,
      x2,
      y2,
      radius,
    );
  }

  rect(x, y, w, h) {
    Path2DInternals.checkInstance(this);
    const prefix = "Failed to execute 'rect' on 'Path2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    op_canvas_2d_path_rect(Path2DInternals.getRaw(this), x, y, w, h);
  }

  roundRect(x, y, w, h, radii = 0) {
    Path2DInternals.checkInstance(this);
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
      Path2DInternals.getRaw(this),
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
    Path2DInternals.checkInstance(this);
    const prefix = "Failed to execute 'arc' on 'Path2D'";
    requiredArguments(arguments.length, 5, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    radius = convertUnrestrictedDouble(radius);
    startAngle = convertUnrestrictedDouble(startAngle);
    endAngle = convertUnrestrictedDouble(endAngle);
    counterclockwise = convertBoolean(counterclockwise);
    if (
      !(NumberIsFinite(x) && NumberIsFinite(y) && NumberIsFinite(radius) &&
        NumberIsFinite(startAngle) && NumberIsFinite(endAngle))
    ) {
      return;
    }
    if (radius < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_ellipse(
      Path2DInternals.getRaw(this),
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
    Path2DInternals.checkInstance(this);
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
    if (
      !(NumberIsFinite(x) && NumberIsFinite(y) &&
        NumberIsFinite(radiusX) && NumberIsFinite(radiusY) &&
        NumberIsFinite(rotation) && NumberIsFinite(startAngle) &&
        NumberIsFinite(endAngle))
    ) {
      return;
    }
    if (radiusX < 0 || radiusY < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_ellipse(
      Path2DInternals.getRaw(this),
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
    ObjectSetPrototypeOf(this, FunctionPrototype);
    configureInterface(this);
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
const OffscreenCanvasRenderingContext2DInternals =
  class OffscreenCanvasRenderingContext2D extends IdentityConstructor {
    #brand() {}

    #canvas;
    #state;
    #alpha;
    #colorSpace;
    #desynchronized;
    #willReadFrequently;
    #cachedDrawingStateStack = ObjectSetPrototypeOf([], null);
    #cachedLang = "inherit";
    #cachedFont = null;
    #cachedLetterSpacing = null;
    #cachedWordSpacing = null;
    #cachedFillStyle = null;
    #cachedStrokeStyle = null;
    #cachedDefaultPath = null;
    #cachedShadowColor = null;
    #cachedFilter = "none";

    constructor(
      o,
      canvas,
      state,
      alpha,
      colorSpace,
      desynchronized,
      willReadFrequently,
    ) {
      super(o);
      this.#canvas = canvas;
      this.#state = state;
      this.#alpha = alpha;
      this.#colorSpace = colorSpace;
      this.#desynchronized = desynchronized;
      this.#willReadFrequently = willReadFrequently;
    }

    static hasInstance(o) {
      // deno-lint-ignore prefer-primordials
      return #brand in o;
    }

    static checkInstance(o) {
      o.#brand;
    }

    static getCanvas(o) {
      return o.#canvas;
    }

    static getState(o) {
      return o.#state;
    }

    static getColorSpace(o) {
      return o.#colorSpace;
    }

    static getSettings(o) {
      return {
        alpha: o.#alpha,
        colorSpace: o.#colorSpace,
        desynchronized: o.#desynchronized,
        willReadFrequently: o.#willReadFrequently,
      };
    }

    static save(o) {
      op_canvas_2d_state_save(o.#state);
      ArrayPrototypePush(o.#cachedDrawingStateStack, {
        lang: o.#cachedLang,
        font: o.#cachedFont,
        letterSpacing: o.#cachedLetterSpacing,
        wordSpacing: o.#cachedWordSpacing,
        fillStyle: o.#cachedFillStyle,
        strokeStyle: o.#cachedStrokeStyle,
        defaultPath: o.#cachedDefaultPath,
        shadowColor: o.#cachedShadowColor,
        filter: o.#cachedFilter,
      });
    }

    static restore(o) {
      const cache = ArrayPrototypePop(o.#cachedDrawingStateStack);
      if (!cache) {
        return;
      }
      op_canvas_2d_state_restore(o.#state);
      o.#cachedLang = cache.lang;
      o.#cachedFont = cache.font;
      o.#cachedLetterSpacing = cache.letterSpacing;
      o.#cachedWordSpacing = cache.wordSpacing;
      o.#cachedFillStyle = cache.fillStyle;
      o.#cachedStrokeStyle = cache.strokeStyle;
      o.#cachedDefaultPath = cache.defaultPath;
      o.#cachedShadowColor = cache.shadowColor;
      o.#cachedFilter = cache.filter;
    }

    static getStrokeStyle(o) {
      o.#cachedStrokeStyle ??= op_canvas_2d_state_stroke_style(o.#state);
      return o.#cachedStrokeStyle;
    }

    static setStrokeStyle(o, value) {
      if (typeof value === "string") {
        if (op_canvas_2d_state_set_stroke_style_color(o.#state, value)) {
          o.#cachedStrokeStyle = null;
        }
      } else {
        if (CanvasGradientInternals.hasInstance(value)) {
          op_canvas_2d_state_set_stroke_style_gradient(
            o.#state,
            CanvasGradientInternals.getRaw(value),
          );
        } else {
          op_canvas_2d_state_set_stroke_style_pattern(
            o.#state,
            CanvasPatternInternals.getRaw(value),
          );
        }
        o.#cachedStrokeStyle = value;
      }
    }

    static getFillStyle(o) {
      o.#cachedFillStyle ??= op_canvas_2d_state_fill_style(o.#state);
      return o.#cachedFillStyle;
    }

    static setFillStyle(o, value) {
      if (typeof value === "string") {
        if (op_canvas_2d_state_set_fill_style_color(o.#state, value)) {
          o.#cachedFillStyle = null;
        }
      } else {
        if (CanvasGradientInternals.hasInstance(value)) {
          op_canvas_2d_state_set_fill_style_gradient(
            o.#state,
            CanvasGradientInternals.getRaw(value),
          );
        } else {
          op_canvas_2d_state_set_fill_style_pattern(
            o.#state,
            CanvasPatternInternals.getRaw(value),
          );
        }
        o.#cachedFillStyle = value;
      }
    }

    static getShadowColor(o) {
      o.#cachedShadowColor ??= op_canvas_2d_state_shadow_color(o.#state);
      return o.#cachedShadowColor;
    }

    static setShadowColor(o, value) {
      if (op_canvas_2d_state_set_shadow_color(o.#state, value)) {
        o.#cachedShadowColor = null;
      }
    }

    static getFilter(o) {
      return o.#cachedFilter;
    }

    static setFilter(o, value) {
      if (op_canvas_2d_state_set_filter(o.#state, value)) {
        o.#cachedFilter = value;
      }
    }

    static getDefaultPath(o) {
      o.#cachedDefaultPath ??= op_canvas_2d_path_new();
      return o.#cachedDefaultPath;
    }

    static getIntendedPath(o, path) {
      return path
        ? Path2DInternals.getRaw(path)
        : OffscreenCanvasRenderingContext2DInternals.getDefaultPath(o);
    }

    static beginPath(o) {
      if (o.#cachedDefaultPath) {
        op_canvas_2d_path_clear(o.#cachedDefaultPath);
      } else {
        o.#cachedDefaultPath = op_canvas_2d_path_new();
      }
    }

    static getLang(o) {
      return o.#cachedLang;
    }

    static setLang(o, value) {
      let locale;
      if (value !== "" && value !== "inherit") {
        try {
          locale = new IntlLocale(value);
          locale = IntlLocalePrototypeMaximize(locale);
        } catch {
          // ignored
        }
      }
      const lang = locale && IntlLocalePrototypeGetBaseName(locale);
      const script = locale && IntlLocalePrototypeGetScript(locale);
      op_canvas_2d_state_set_lang(o.#state, lang ?? "", script ?? "");
      o.#cachedLang = value;
    }

    static getFont(o) {
      o.#cachedFont ??= op_canvas_2d_state_font(o.#state);
      return o.#cachedFont;
    }

    static setFont(o, value) {
      if (op_canvas_2d_state_set_font(o.#state, value)) {
        o.#cachedFont = null;
      }
    }

    static invalidateCachedFont(o) {
      o.#cachedFont = null;
    }

    static getLetterSpacing(o) {
      o.#cachedLetterSpacing ??= op_canvas_2d_state_letter_spacing(o.#state);
      return o.#cachedLetterSpacing;
    }

    static setLetterSpacing(o, value) {
      if (op_canvas_2d_state_set_letter_spacing(o.#state, value)) {
        o.#cachedLetterSpacing = null;
      }
    }

    static getWordSpacing(o) {
      o.#cachedWordSpacing ??= op_canvas_2d_state_word_spacing(o.#state);
      return o.#cachedWordSpacing;
    }

    static setWordSpacing(o, value) {
      if (op_canvas_2d_state_set_word_spacing(o.#state, value)) {
        o.#cachedWordSpacing = null;
      }
    }

    static inspect(inspect, options) {
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
  };
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

export class OffscreenCanvasRenderingContext2D extends Object {
  // deno-lint-ignore constructor-super
  constructor() {
    illegalConstructor();
  }

  get canvas() {
    return OffscreenCanvasRenderingContext2DInternals.getCanvas(this);
  }

  getContextAttributes() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getSettings(this);
  }

  save() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    OffscreenCanvasRenderingContext2DInternals.save(this);
  }

  restore() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    OffscreenCanvasRenderingContext2DInternals.restore(this);
  }

  reset() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    op_canvas_2d_state_reset(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  isContextLost() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return false;
  }

  scale(x, y) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'scale' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_state_scale(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      x,
      y,
    );
  }

  rotate(angle) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'rotate' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    angle = convertUnrestrictedDouble(angle);
    op_canvas_2d_state_rotate(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      angle,
    );
  }

  translate(x, y) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'translate' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_state_translate(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      x,
      y,
    );
  }

  transform(a, b, c, d, e, f) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'transform' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 6, prefix);
    a = convertUnrestrictedDouble(a);
    b = convertUnrestrictedDouble(b);
    c = convertUnrestrictedDouble(c);
    d = convertUnrestrictedDouble(d);
    e = convertUnrestrictedDouble(e);
    f = convertUnrestrictedDouble(f);
    op_canvas_2d_state_transform(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      a,
      b,
      c,
      d,
      e,
      f,
    );
  }

  getTransform() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    op_canvas_2d_state_get_transform(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      getTransformBuffer,
    );
    return createDOMMatrix(getTransformBuffer, true);
  }

  setTransform(a = undefined, b, c, d, e, f) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    op_canvas_2d_state_set_transform(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      a,
      b,
      c,
      d,
      e,
      f,
    );
  }

  resetTransform() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    op_canvas_2d_state_reset_transform(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  get globalAlpha() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_global_alpha(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  set globalAlpha(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'globalAlpha' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_global_alpha(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      value,
    );
  }

  get globalCompositeOperation() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return blendOrCompositeModeFromRepr[
      op_canvas_2d_state_global_composite_operation(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set globalCompositeOperation(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'globalCompositeOperation' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = blendOrCompositeModeToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_global_composite_operation(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get imageSmoothingEnabled() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_image_smoothing_enabled(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  set imageSmoothingEnabled(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'imageSmoothingEnabled' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertBoolean(value);
    op_canvas_2d_state_set_image_smoothing_enabled(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      value,
    );
  }

  get imageSmoothingQuality() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return imageSmoothingQualityFromRepr[
      op_canvas_2d_state_image_smoothing_quality(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set imageSmoothingQuality(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'imageSmoothingQuality' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = imageSmoothingQualityToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_image_smoothing_quality(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get strokeStyle() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getStrokeStyle(this);
  }

  set strokeStyle(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'strokeStyle' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMStringOrCanvasGradientOrCanvasPattern(value);
    OffscreenCanvasRenderingContext2DInternals.setStrokeStyle(this, value);
  }

  get fillStyle() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getFillStyle(this);
  }

  set fillStyle(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'fillStyle' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMStringOrCanvasGradientOrCanvasPattern(value);
    OffscreenCanvasRenderingContext2DInternals.setFillStyle(this, value);
  }

  createLinearGradient(x0, y0, x1, y1) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'createConicGradient' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 3, prefix);
    startAngle = convertDouble(startAngle);
    x = convertDouble(x);
    y = convertDouble(y);
    return conicGradient(startAngle, x, y);
  }

  createPattern(image, repetition) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'createPattern' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    image = convertCanvasImageSource(image);
    repetition = convertLegacyNullToEmptyStringDOMString(repetition);
    return pattern(image, repetition);
  }

  get shadowOffsetX() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_shadow_offset_x(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  set shadowOffsetX(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'shadowOffsetX' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_shadow_offset_x(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      value,
    );
  }

  get shadowOffsetY() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_shadow_offset_y(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  set shadowOffsetY(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'shadowOffsetY' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_shadow_offset_y(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      value,
    );
  }

  get shadowBlur() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_shadow_blur(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  set shadowBlur(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'shadowBlur' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_shadow_blur(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      value,
    );
  }

  get shadowColor() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getShadowColor(this);
  }

  set shadowColor(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'shadowColor' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    OffscreenCanvasRenderingContext2DInternals.setShadowColor(this, value);
  }

  get filter() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getFilter(this);
  }

  set filter(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'filter' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    OffscreenCanvasRenderingContext2DInternals.setFilter(this, value);
  }

  clearRect(x, y, w, h) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    op_canvas_2d_state_clear_rect(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      x,
      y,
      w,
      h,
    );
  }

  fillRect(x, y, w, h) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    op_canvas_2d_state_fill_rect(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      x,
      y,
      w,
      h,
    );
  }

  strokeRect(x, y, w, h) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    op_canvas_2d_state_stroke_rect(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      x,
      y,
      w,
      h,
    );
  }

  beginPath() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    OffscreenCanvasRenderingContext2DInternals.beginPath(this);
  }

  fill(path = undefined, fillRule) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const nArgs = arguments.length;
    if (
      nArgs === 0 ||
      (nArgs === 1 &&
        !(type(path) === "Object" && Path2DInternals.hasInstance(path)))
    ) {
      fillRule = path;
      path = null;
    } else {
      path = convertPath2D(path);
    }
    fillRule = convertCanvasFillRule(defaultTo(fillRule, "nonzero"));
    op_canvas_2d_state_fill(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      OffscreenCanvasRenderingContext2DInternals.getIntendedPath(this, path),
      fillRuleToRepr[fillRule],
    );
  }

  stroke(path = undefined) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    path = arguments.length === 0 ? null : convertPath2D(path);
    op_canvas_2d_state_stroke(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      OffscreenCanvasRenderingContext2DInternals.getIntendedPath(this, path),
    );
  }

  clip(path = undefined, fillRule) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const nArgs = arguments.length;
    if (
      nArgs === 0 ||
      (nArgs === 1 &&
        !(type(path) === "Object" && Path2DInternals.hasInstance(path)))
    ) {
      fillRule = path;
      path = null;
    } else {
      path = convertPath2D(path);
    }
    fillRule = convertCanvasFillRule(defaultTo(fillRule, "nonzero"));
    op_canvas_2d_state_clip(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      OffscreenCanvasRenderingContext2DInternals.getIntendedPath(this, path),
      fillRuleToRepr[fillRule],
    );
  }

  isPointInPath(path, x, y = undefined, fillRule) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const nArgs = arguments.length;
    const prefix =
      "Failed to execute 'isPointInPath' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(nArgs, 2, prefix);
    if (
      nArgs === 2 ||
      (nArgs === 3 &&
        !(type(path) === "Object" && Path2DInternals.hasInstance(path)))
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
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      OffscreenCanvasRenderingContext2DInternals.getIntendedPath(this, path),
      x,
      y,
      fillRuleToRepr[fillRule],
    );
  }

  isPointInStroke(path, x, y = undefined) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      OffscreenCanvasRenderingContext2DInternals.getIntendedPath(this, path),
      x,
      y,
    );
  }

  fillText(text, x, y, maxWidth = undefined) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    op_canvas_2d_state_fill_text(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      text,
      x,
      y,
      maxWidth,
    );
  }

  strokeText(text, x, y, maxWidth = undefined) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    op_canvas_2d_state_stroke_text(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      text,
      x,
      y,
      maxWidth,
    );
  }

  measureText(text) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'measureText' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    text = convertDOMString(text);
    op_canvas_2d_state_measure_text(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      text,
      measureTextBuffer,
    );
    return createTextMetrics(measureTextBuffer);
  }

  drawImage(image, sx, sy, sw = undefined, sh, dx, dy, dw, dh) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
      if (!(NumberIsFinite(dx) && NumberIsFinite(dy))) {
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
      if (
        !(NumberIsFinite(dx) && NumberIsFinite(dy) &&
          NumberIsFinite(dw) && NumberIsFinite(dh))
      ) {
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
      if (
        !(NumberIsFinite(sx) && NumberIsFinite(sy) &&
          NumberIsFinite(sw) && NumberIsFinite(sh) &&
          NumberIsFinite(dx) && NumberIsFinite(dy) &&
          NumberIsFinite(dw) && NumberIsFinite(dh))
      ) {
        return;
      }
    } else {
      throw new TypeError("Overload resolution failed");
    }
    const bitmap = checkUsabilityAndClone(image);
    sw ??= op_canvas_2d_image_bitmap_width(bitmap);
    sh ??= op_canvas_2d_image_bitmap_height(bitmap);
    op_canvas_2d_state_draw_image(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
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
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
      const colorSpace = settings.colorSpace ??
        OffscreenCanvasRenderingContext2DInternals.getColorSpace(this);
      return new ImageData(sw, sh, { __proto__: null, colorSpace });
    }
  }

  getImageData(sx, sy, sw, sh, settings = undefined) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    const colorSpace = settings.colorSpace ??
      OffscreenCanvasRenderingContext2DInternals.getColorSpace(this);
    const result = new ImageData(sw, sh, { __proto__: null, colorSpace });
    const buf = new Uint32Array(
      TypedArrayPrototypeGetBuffer(ImageDataPrototypeGetData(result)),
    );
    op_canvas_2d_state_get_image_data(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
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
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
      OffscreenCanvasRenderingContext2DInternals.getState(this),
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
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_line_width(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  set lineWidth(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'lineWidth' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_line_width(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      value,
    );
  }

  get lineCap() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return lineCapFromRepr[
      op_canvas_2d_state_line_cap(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set lineCap(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'lineCap' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = lineCapToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_line_cap(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get lineJoin() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return lineJoinFromRepr[
      op_canvas_2d_state_line_join(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set lineJoin(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'lineJoin' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = lineJoinToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_line_join(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get miterLimit() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_miter_limit(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  set miterLimit(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'miterLimit' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_miter_limit(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      value,
    );
  }

  setLineDash(segments) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    op_canvas_2d_state_set_dash_list(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      buf,
    );
  }

  getLineDash() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_dash_list(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  get lineDashOffset() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return op_canvas_2d_state_line_dash_offset(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
    );
  }

  set lineDashOffset(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'lineDashOffset' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertUnrestrictedDouble(value);
    op_canvas_2d_state_set_line_dash_offset(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      value,
    );
  }

  get lang() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getLang(this);
  }

  set lang(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'lang' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    OffscreenCanvasRenderingContext2DInternals.setLang(this, value);
  }

  get font() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getFont(this);
  }

  set font(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'font' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    OffscreenCanvasRenderingContext2DInternals.setFont(this, value);
  }

  get textAlign() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return textAlignFromRepr[
      op_canvas_2d_state_text_align(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set textAlign(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'textAlign' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = textAlignToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_text_align(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get textBaseline() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return textBaselineFromRepr[
      op_canvas_2d_state_text_baseline(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set textBaseline(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'textBaseline' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = textBaselineToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_text_baseline(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get direction() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return directionFromRepr[
      op_canvas_2d_state_direction(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set direction(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'direction' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = directionToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_direction(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get letterSpacing() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getLetterSpacing(this);
  }

  set letterSpacing(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'letterSpacing' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    OffscreenCanvasRenderingContext2DInternals.setLetterSpacing(this, value);
  }

  get fontKerning() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return fontKerningFromRepr[
      op_canvas_2d_state_font_kerning(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set fontKerning(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'fontKerning' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = fontKerningToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_font_kerning(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get fontStretch() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return fontStretchFromRepr[
      op_canvas_2d_state_font_stretch(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set fontStretch(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'fontStretch' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = fontStretchToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_font_stretch(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
    OffscreenCanvasRenderingContext2DInternals.invalidateCachedFont(this);
  }

  get fontVariantCaps() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return fontVariantCapsFromRepr[
      op_canvas_2d_state_font_variant_caps(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set fontVariantCaps(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'fontVariantCaps' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = fontVariantCapsToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_font_variant_caps(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
    OffscreenCanvasRenderingContext2DInternals.invalidateCachedFont(this);
  }

  get textRendering() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return textRenderingFromRepr[
      op_canvas_2d_state_text_rendering(
        OffscreenCanvasRenderingContext2DInternals.getState(this),
      )
    ];
  }

  set textRendering(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'textRendering' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    const repr = textRenderingToRepr[value];
    if (repr === undefined) {
      return;
    }
    op_canvas_2d_state_set_text_rendering(
      OffscreenCanvasRenderingContext2DInternals.getState(this),
      repr,
    );
  }

  get wordSpacing() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    return OffscreenCanvasRenderingContext2DInternals.getWordSpacing(this);
  }

  set wordSpacing(value) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to set 'wordSpacing' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    OffscreenCanvasRenderingContext2DInternals.setWordSpacing(this, value);
  }

  closePath() {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    op_canvas_2d_path_close(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
    );
  }

  moveTo(x, y) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'moveTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_move_to(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
      x,
      y,
    );
  }

  lineTo(x, y) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'lineTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 2, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_line_to(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
      x,
      y,
    );
  }

  quadraticCurveTo(cpx, cpy, x, y) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'quadraticCurveTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    cpx = convertUnrestrictedDouble(cpx);
    cpy = convertUnrestrictedDouble(cpy);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_quad_to(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
      cpx,
      cpy,
      x,
      y,
    );
  }

  bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'bezierCurveTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 6, prefix);
    cp1x = convertUnrestrictedDouble(cp1x);
    cp1y = convertUnrestrictedDouble(cp1y);
    cp2x = convertUnrestrictedDouble(cp2x);
    cp2y = convertUnrestrictedDouble(cp2y);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    op_canvas_2d_path_cubic_to(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
      cp1x,
      cp1y,
      cp2x,
      cp2y,
      x,
      y,
    );
  }

  arcTo(x1, y1, x2, y2, radius) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'arcTo' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 5, prefix);
    x1 = convertUnrestrictedDouble(x1);
    y1 = convertUnrestrictedDouble(y1);
    x2 = convertUnrestrictedDouble(x2);
    y2 = convertUnrestrictedDouble(y2);
    radius = convertUnrestrictedDouble(radius);
    if (
      !(NumberIsFinite(x1) && NumberIsFinite(y1) &&
        NumberIsFinite(x2) && NumberIsFinite(y2) && NumberIsFinite(radius))
    ) {
      return;
    }
    if (radius < 0) {
      op_canvas_2d_path_ensure_subpath(
        OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
        x1,
        y1,
      );
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_arc_to(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
      x1,
      y1,
      x2,
      y2,
      radius,
    );
  }

  rect(x, y, w, h) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'rect' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 4, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    w = convertUnrestrictedDouble(w);
    h = convertUnrestrictedDouble(h);
    op_canvas_2d_path_rect(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
      x,
      y,
      w,
      h,
    );
  }

  roundRect(x, y, w, h, radii = 0) {
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
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
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'arc' on 'OffscreenCanvasRenderingContext2D'";
    requiredArguments(arguments.length, 5, prefix);
    x = convertUnrestrictedDouble(x);
    y = convertUnrestrictedDouble(y);
    radius = convertUnrestrictedDouble(radius);
    startAngle = convertUnrestrictedDouble(startAngle);
    endAngle = convertUnrestrictedDouble(endAngle);
    counterclockwise = convertBoolean(counterclockwise);
    if (
      !(NumberIsFinite(x) && NumberIsFinite(y) && NumberIsFinite(radius) &&
        NumberIsFinite(startAngle) && NumberIsFinite(endAngle))
    ) {
      return;
    }
    if (radius < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_ellipse(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
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
    OffscreenCanvasRenderingContext2DInternals.checkInstance(this);
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
    if (
      !(NumberIsFinite(x) && NumberIsFinite(y) &&
        NumberIsFinite(radiusX) && NumberIsFinite(radiusY) &&
        NumberIsFinite(rotation) && NumberIsFinite(startAngle) &&
        NumberIsFinite(endAngle))
    ) {
      return;
    }
    if (radiusX < 0 || radiusY < 0) {
      throw new DOMException("Radius must be non-negative", "IndexSizeError");
    }
    op_canvas_2d_path_ellipse(
      OffscreenCanvasRenderingContext2DInternals.getDefaultPath(this),
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

  get [privateCustomInspect]() {
    return OffscreenCanvasRenderingContext2DInternals.hasInstance(this)
      ? OffscreenCanvasRenderingContext2DInternals.inspect
      : undefined;
  }

  static {
    ObjectSetPrototypeOf(this, FunctionPrototype);
    configureInterface(this);
  }
}

function createOffscreenCanvasRenderingContext2D(
  canvas,
  state,
  alpha,
  colorSpace,
  desynchronized,
  willReadFrequently,
) {
  const o = ObjectCreate(OffscreenCanvasRenderingContext2D.prototype);
  new OffscreenCanvasRenderingContext2DInternals(
    o,
    canvas,
    state,
    alpha,
    colorSpace,
    desynchronized,
    willReadFrequently,
  );
  return o;
}

registerCanvasContextMode("2d", {
  newInstance(canvas, width, height, options) {
    const settings = convertCanvasRenderingContext2DSettings(options);
    return createOffscreenCanvasRenderingContext2D(
      canvas,
      op_canvas_2d_state_new(
        width,
        height,
        settings.alpha,
        colorSpaceToRepr[settings.colorSpace],
      ),
      settings.alpha,
      settings.colorSpace,
      settings.desynchronized,
      settings.willReadFrequently,
    );
  },
  hasInstance(ctx) {
    return OffscreenCanvasRenderingContext2DInternals.hasInstance(ctx);
  },
  getWidth(ctx) {
    const state = OffscreenCanvasRenderingContext2DInternals.getState(ctx);
    return op_canvas_2d_state_width(state);
  },
  setWidth(ctx, width) {
    const state = OffscreenCanvasRenderingContext2DInternals.getState(ctx);
    op_canvas_2d_state_set_width(state, width);
  },
  getHeight(ctx) {
    const state = OffscreenCanvasRenderingContext2DInternals.getState(ctx);
    return op_canvas_2d_state_height(state);
  },
  setHeight(ctx, height) {
    const state = OffscreenCanvasRenderingContext2DInternals.getState(ctx);
    op_canvas_2d_state_set_height(state, height);
  },
  transferToImageBitmap(ctx) {
    const state = OffscreenCanvasRenderingContext2DInternals.getState(ctx);
    const bitmap = op_canvas_2d_image_bitmap_from_canvas_state(state);
    op_canvas_2d_state_clear(state);
    return bitmap;
  },
  cloneToImageBitmap(ctx) {
    const state = OffscreenCanvasRenderingContext2DInternals.getState(ctx);
    return op_canvas_2d_image_bitmap_from_canvas_state(state);
  },
  cropToImageBitmap(ctx, sx, sy, sw, sh) {
    const state = OffscreenCanvasRenderingContext2DInternals.getState(ctx);
    return {
      bitmap: op_canvas_2d_image_bitmap_from_canvas_state_crop(
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
    colorSpace ??= OffscreenCanvasRenderingContext2DInternals
      .getColorSpace(ctx);
    const state = OffscreenCanvasRenderingContext2DInternals.getState(ctx);
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
