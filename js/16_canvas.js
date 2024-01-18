import { primordials } from "ext:core/mod.js";
import {
  ImageDataPrototypeGetColorSpace,
  ImageDataPrototypeGetData,
  ImageDataPrototypeGetHeight,
  ImageDataPrototypeGetWidth,
} from "ext:deno_canvas_2d/00_image_data_primordials.js";
import {
  op_canvas_2d_encode_png,
  op_canvas_2d_image_bitmap_clone,
  op_canvas_2d_image_bitmap_close,
  op_canvas_2d_image_bitmap_crop,
  op_canvas_2d_image_bitmap_empty,
  op_canvas_2d_image_bitmap_empty_resize,
  op_canvas_2d_image_bitmap_from_image_data_cropped,
  op_canvas_2d_image_bitmap_height,
  op_canvas_2d_image_bitmap_resize,
  op_canvas_2d_image_bitmap_width,
} from "ext:deno_canvas_2d/00_ops.js";
import { makeSafePromise } from "ext:deno_canvas_2d/01_promise.js";
import { isBlob } from "ext:deno_canvas_2d/02_is_blob.js";
import { isImageData } from "ext:deno_canvas_2d/02_is_image_data.js";
import { createDictionaryConverter } from "ext:deno_canvas_2d/04_create_dictionary_converter.js";
import { createEnumConverter } from "ext:deno_canvas_2d/04_create_enum_converter.js";
import { convertDOMString } from "ext:deno_canvas_2d/05_convert_dom_string.js";
import { convertEnforceRangeUnsignedLong } from "ext:deno_canvas_2d/05_convert_enforce_range_unsigned_long.js";
import { convertEnforceRangeUnsignedLongLong } from "ext:deno_canvas_2d/05_convert_enforce_range_unsigned_long_long.js";
import { convertEventHandler } from "ext:deno_canvas_2d/05_convert_event_handler.js";
import { convertLong } from "ext:deno_canvas_2d/05_convert_long.js";
import { convertUnrestrictedDouble } from "ext:deno_canvas_2d/05_convert_unrestricted_double.js";
import { EventHandler } from "ext:deno_canvas_2d/15_event.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import { EventTarget } from "ext:deno_web/02_event.js";
import { defer } from "ext:deno_web/02_timers.js";
import { Blob } from "ext:deno_web/09_file.js";
import {
  configureInterface,
  illegalConstructor,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";

const {
  MathMin,
  ObjectFreeze,
  ObjectGetPrototypeOf,
  Promise,
  RangeError,
  SafeArrayIterator,
  SafeMap,
  SymbolFor,
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
const privateCustomInspect = SymbolFor("Deno.privateCustomInspect");
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
const contextModes = new SafeMap();

export function registerCanvasContextMode(id, mode) {
  if (contextModes.has(id)) {
    throw new TypeError(`Context mode '${id}' has already been registered`);
  }
  contextModes.set(id, mode);
}

registerCanvasContextMode("none", {
  newInstance() {
    throw new TypeError("Unreachable");
  },
  hasInstance(ctx) {
    return objectIsDummyCanvasContext(ctx);
  },
  getWidth(ctx) {
    return ctx.width;
  },
  setWidth(ctx, value) {
    ctx.width = value;
  },
  getHeight(ctx) {
    return ctx.height;
  },
  setHeight(ctx, value) {
    ctx.height = value;
  },
  transferToImageBitmap() {
    throw new DOMException("Canvas has no context", "InvalidStateError");
  },
  cloneToImageBitmap(ctx) {
    return op_canvas_2d_image_bitmap_empty(
      MathMin(ctx.width, 0xffffffff),
      MathMin(ctx.height, 0xffffffff),
    );
  },
  cropToImageBitmap(ctx, _sx, _sy, sw, sh, dw, dh) {
    return {
      bitmap: op_canvas_2d_image_bitmap_empty_resize(
        sw ?? ctx.width,
        sh ?? ctx.height,
        dw ?? 0,
        dh ?? 0,
      ),
      needResize: false,
    };
  },
  getDataForSerialization(ctx, colorSpace) {
    colorSpace ??= "srgb";
    const { width, height } = ctx;
    const data = new Uint8Array(width * height * 4);
    return { data, width, height, colorSpace };
  },
});
export let objectIsOffscreenCanvas;
let getOffscreenCanvasContext;
let getOffscreenCanvasWidth;
let getOffscreenCanvasHeight;

function getOffscreenCanvasContextMode(ctx) {
  // deno-lint-ignore prefer-primordials
  for (const mode of contextModes.values()) {
    if (mode.hasInstance(ctx)) {
      return mode;
    }
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

  get #width() {
    const ctx = this.#context;
    if (!ctx) {
      return 0;
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    return mode.getWidth(ctx);
  }

  set #width(value) {
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    mode.setWidth(value);
  }

  get #height() {
    const ctx = this.#context;
    if (!ctx) {
      return 0;
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    return mode.getHeight(ctx);
  }

  set #height(value) {
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    mode.setHeight(value);
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
    contextId = convertDOMString(contextId);
    const requestedMode = contextModes.get(contextId);
    if (!requestedMode || contextId === "none") {
      throw new TypeError(`Invalid rendering context ID '${contextId}'`);
    }
    if (!(typeof options === "object" || typeof options === "function")) {
      options = null;
    }
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    if (objectIsDummyCanvasContext(ctx)) {
      const { width, height } = ctx;
      return this.#context = requestedMode
        .newInstance(this, width, height, options);
    }
    return requestedMode.hasInstance(ctx) ? ctx : null;
  }

  transferToImageBitmap() {
    this.#brand;
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    return mode.transferToImageBitmap(ctx);
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
    const ctx = this.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const width = this.#width;
    const height = this.#height;
    if (width === 0 || height === 0) {
      throw new DOMException("Canvas has no pixels", "IndexSizeError");
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    switch (type) {
      default: {
        const result = mode.getDataForSerialization(ctx, null);
        return {
          data: op_canvas_2d_encode_png(
            result.data,
            result.width,
            result.height,
            colorSpaceToRepr[result.colorSpace],
          ),
          type: "image/png",
        };
      }
    }
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
export const alignUint8ClampedArrayToUint32 = (data) => {
  const offset = TypedArrayPrototypeGetByteOffset(data);
  const length = TypedArrayPrototypeGetByteLength(data);
  return offset % 4 === 0
    ? new Uint32Array(TypedArrayPrototypeGetBuffer(data), offset, length / 4)
    : new Uint32Array(
      TypedArrayPrototypeGetBuffer(new Uint8ClampedArray(data)),
    );
};
export const colorSpaceToRepr = ObjectFreeze({
  __proto__: null,
  "srgb": 0,
  "display-p3": 1,
});
export let objectIsImageBitmap;
export let getImageBitmapRaw;

export class ImageBitmap {
  #brand() {}

  #raw;

  constructor(key = undefined, raw) {
    if (key !== illegalConstructor) {
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
  }
}

export const checkUsabilityAndClone = (image) => {
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
  const mode = getOffscreenCanvasContextMode(ctx);
  return mode.cloneToImageBitmap(ctx);
};
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
    const mode = getOffscreenCanvasContextMode(ctx);
    const result = mode.cropToImageBitmap(ctx, sx, sy, sw, sh, dw, dh);
    let bitmap = result.bitmap;
    if (result.needResize) {
      bitmap = op_canvas_2d_image_bitmap_resize(
        bitmap,
        dw ?? 0,
        dh ?? 0,
        resizeQualityToRepr[resizeQuality],
        imageOrientationToRepr[imageOrientation],
      );
    }
    return bitmap;
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
  async function createImageBitmap(image, sx = undefined, sy, sw, sh, options) {
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
    return new ImageBitmap(illegalConstructor, bitmap);
  };
