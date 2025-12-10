import { primordials } from "ext:core/mod.js";
import { op_defer } from "ext:core/ops";
import { createFilteredInspectProxy } from "ext:deno_web/01_console.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import { EventTarget } from "ext:deno_web/02_event.js";
import { Blob } from "ext:deno_web/09_file.js";
import {
  configureInterface,
  illegalConstructor,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";
import {
  BlobPrototypeBytes,
  BlobPrototypeGetType,
} from "./00_blob_primordials.js";
import {
  ImageDataPrototypeGetColorSpace,
  ImageDataPrototypeGetData,
  ImageDataPrototypeGetHeight,
  ImageDataPrototypeGetWidth,
} from "./00_image_data_primordials.js";
import {
  op_canvas_2d_decode_image,
  op_canvas_2d_encode_png,
  op_canvas_2d_image_bitmap_clone,
  op_canvas_2d_image_bitmap_close,
  op_canvas_2d_image_bitmap_crop,
  op_canvas_2d_image_bitmap_empty,
  op_canvas_2d_image_bitmap_empty_resize,
  op_canvas_2d_image_bitmap_from_image_data_crop_and_resize,
  op_canvas_2d_image_bitmap_height,
  op_canvas_2d_image_bitmap_resize,
  op_canvas_2d_image_bitmap_width,
} from "./00_ops.js";
import { capturePrototype } from "./01_capture_prototype.js";
import { IdentityConstructor } from "./01_identity_constructor.js";
import { makeSafePromise } from "./01_promise.js";
import { isBlob } from "./02_is_blob.js";
import { isImageData } from "./02_is_image_data.js";
import { createDictionaryConverter } from "./04_create_dictionary_converter.js";
import { createEnumConverter } from "./04_create_enum_converter.js";
import { convertDOMString } from "./05_convert_dom_string.js";
import { convertEnforceRangeUnsignedLong } from "./05_convert_enforce_range_unsigned_long.js";
import { convertEnforceRangeUnsignedLongLong } from "./05_convert_enforce_range_unsigned_long_long.js";
import { convertEventHandler } from "./05_convert_event_handler.js";
import { convertLong } from "./05_convert_long.js";
import { convertUnrestrictedDouble } from "./05_convert_unrestricted_double.js";
import { EventHandler } from "./15_event.js";

const {
  FunctionPrototype,
  Object,
  ObjectCreate,
  ObjectFreeze,
  ObjectGetPrototypeOf,
  ObjectSetPrototypeOf,
  PromiseReject,
  RangeError,
  ReflectConstruct,
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
  setWidth(ctx, width) {
    ctx.width = width;
  },
  getHeight(ctx) {
    return ctx.height;
  },
  setHeight(ctx, height) {
    ctx.height = height;
  },
  transferToImageBitmap() {
    throw new DOMException("Canvas has no context", "InvalidStateError");
  },
  cloneToImageBitmap() {
    return op_canvas_2d_image_bitmap_empty(1, 1);
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
    const data = TypedArrayPrototypeGetBuffer(new Uint32Array(width * height));
    return { data, width, height, colorSpace };
  },
});

export function getOffscreenCanvasContextMode(ctx) {
  // deno-lint-ignore prefer-primordials
  for (const mode of contextModes.values()) {
    if (mode.hasInstance(ctx)) {
      return mode;
    }
  }
  throw new TypeError("Unreachable");
}

export const OffscreenCanvasInternals = class OffscreenCanvas
  extends IdentityConstructor {
  #brand() {}

  #context;
  #oncontextlost = new EventHandler(this, "contextlost");
  #oncontextrestored = new EventHandler(this, "contextrestored");

  constructor(o, context) {
    super(o);
    this.#context = context;
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getContext(o) {
    return o.#context;
  }

  static setContext(o, value) {
    o.#context = value;
  }

  static getOnContextLost(o) {
    return o.#oncontextlost;
  }

  static getOnContextRestored(o) {
    return o.#oncontextrestored;
  }

  static getWidth(o) {
    const ctx = o.#context;
    if (!ctx) {
      return 0;
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    return mode.getWidth(ctx);
  }

  static setWidth(o, value) {
    const ctx = o.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    mode.setWidth(value);
  }

  static getHeight(o) {
    const ctx = o.#context;
    if (!ctx) {
      return 0;
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    return mode.getHeight(ctx);
  }

  static setHeight(o, value) {
    const ctx = o.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    mode.setHeight(value);
  }

  static encode(o, type) {
    const ctx = o.#context;
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const width = OffscreenCanvasInternals.getWidth(o);
    const height = OffscreenCanvasInternals.getHeight(o);
    if (width === 0 || height === 0) {
      throw new DOMException("Canvas has no pixels", "IndexSizeError");
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    switch (type) {
      default: {
        const result = mode.getDataForSerialization(ctx, null);
        return {
          data: op_canvas_2d_encode_png(
            new Uint8Array(result.data),
            result.width,
            result.height,
            colorSpaceToRepr[result.colorSpace],
          ),
          type: "image/png",
        };
      }
    }
  }

  static async convertToBlob(o, options) {
    try {
      const { data, type } = OffscreenCanvasInternals.encode(o, options.type);
      return new Blob(new SafeArrayIterator([data]), { __proto__: null, type });
    } finally {
      await makeSafePromise(op_defer());
    }
  }

  static inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["width", "height", "oncontextlost", "oncontextrestored"],
      }),
      options,
    );
  }
};

export class OffscreenCanvas extends EventTarget {
  constructor(width, height) {
    const prefix = "Failed to construct 'OffscreenCanvas'";
    requiredArguments(arguments.length, 2, prefix);
    width = convertEnforceRangeUnsignedLongLong(width);
    height = convertEnforceRangeUnsignedLongLong(height);
    const newTarget = capturePrototype(new.target, OffscreenCanvas);
    const o = ReflectConstruct(EventTarget, [], newTarget);
    new OffscreenCanvasInternals(o, new DummyCanvasContext(width, height));
    return o;
  }

  get width() {
    return OffscreenCanvasInternals.getWidth(this);
  }

  set width(value) {
    OffscreenCanvasInternals.checkInstance(this);
    value = convertEnforceRangeUnsignedLongLong(value);
    OffscreenCanvasInternals.setWidth(this, value);
  }

  get height() {
    return OffscreenCanvasInternals.getHeight(this);
  }

  set height(value) {
    OffscreenCanvasInternals.checkInstance(this);
    value = convertEnforceRangeUnsignedLongLong(value);
    OffscreenCanvasInternals.setHeight(this, value);
  }

  getContext(contextId, options = null) {
    OffscreenCanvasInternals.checkInstance(this);
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
    const ctx = OffscreenCanvasInternals.getContext(this);
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    if (objectIsDummyCanvasContext(ctx)) {
      const { width, height } = ctx;
      const result = requestedMode.newInstance(this, width, height, options);
      OffscreenCanvasInternals.setContext(this, result);
      return result;
    }
    return requestedMode.hasInstance(ctx) ? ctx : null;
  }

  transferToImageBitmap() {
    OffscreenCanvasInternals.checkInstance(this);
    const ctx = OffscreenCanvasInternals.getContext(this);
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const mode = getOffscreenCanvasContextMode(ctx);
    return createImageBitmapFromRaw(mode.transferToImageBitmap(ctx));
  }

  convertToBlob(options = undefined) {
    try {
      OffscreenCanvasInternals.checkInstance(this);
      options = convertImageEncodeOptions(options);
      return OffscreenCanvasInternals.convertToBlob(this, options);
    } catch (e) {
      return PromiseReject(e);
    }
  }

  get oncontextlost() {
    return OffscreenCanvasInternals.getOnContextLost(this).value;
  }

  set oncontextlost(value) {
    OffscreenCanvasInternals.checkInstance(this);
    value = convertEventHandler(value);
    OffscreenCanvasInternals.getOnContextLost(this).update(value);
  }

  get oncontextrestored() {
    return OffscreenCanvasInternals.getOnContextRestored(this).value;
  }

  set oncontextrestored(value) {
    OffscreenCanvasInternals.checkInstance(this);
    value = convertEventHandler(value);
    OffscreenCanvasInternals.getOnContextRestored(this).update(value);
  }

  get [privateCustomInspect]() {
    return OffscreenCanvasInternals.hasInstance(this)
      ? OffscreenCanvasInternals.inspect
      : undefined;
  }

  static {
    configureInterface(this);
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
export const colorSpaceFromRepr = ObjectFreeze([
  "srgb",
  "display-p3",
]);
export const colorSpaceToRepr = ObjectFreeze({
  __proto__: null,
  "srgb": 0,
  "display-p3": 1,
});
export const ImageBitmapInternals = class ImageBitmap
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

  static transfer(o) {
    const bitmap = o.#raw;
    o.#raw = null;
    return bitmap;
  }

  static inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["width", "height"],
      }),
      options,
    );
  }
};

export class ImageBitmap extends Object {
  // deno-lint-ignore constructor-super
  constructor() {
    illegalConstructor();
  }

  get width() {
    const raw = ImageBitmapInternals.getRaw(this);
    return raw ? op_canvas_2d_image_bitmap_width(raw) : 0;
  }

  get height() {
    const raw = ImageBitmapInternals.getRaw(this);
    return raw ? op_canvas_2d_image_bitmap_height(raw) : 0;
  }

  close() {
    // deno-lint-ignore prefer-primordials
    const bitmap = ImageBitmapInternals.transfer(this);
    if (bitmap) {
      op_canvas_2d_image_bitmap_close(bitmap);
    }
  }

  get [privateCustomInspect]() {
    return ImageBitmapInternals.hasInstance(this)
      ? ImageBitmapInternals.inspect
      : undefined;
  }

  static {
    ObjectSetPrototypeOf(this, FunctionPrototype);
    configureInterface(this);
  }
}

export function createImageBitmapFromRaw(raw) {
  const o = ObjectCreate(ImageBitmap.prototype);
  new ImageBitmapInternals(o, raw);
  return o;
}

export function checkUsabilityAndClone(image) {
  if (ImageBitmapInternals.hasInstance(image)) {
    const raw = ImageBitmapInternals.getRaw(image);
    if (!raw) {
      throw new DOMException("Image is detached", "InvalidStateError");
    }
    return op_canvas_2d_image_bitmap_clone(raw);
  }
  const ctx = OffscreenCanvasInternals.getContext(image);
  if (!ctx) {
    throw new DOMException("Canvas is detached", "InvalidStateError");
  }
  const width = OffscreenCanvasInternals.getWidth(image);
  const height = OffscreenCanvasInternals.getHeight(image);
  if (width === 0 || height === 0) {
    throw new DOMException("Canvas has no pixels", "InvalidStateError");
  }
  const mode = getOffscreenCanvasContextMode(ctx);
  return mode.cloneToImageBitmap(ctx);
}

const convertImageBitmapSource = (value) => {
  if (
    (type(value) === "Object" &&
      (ImageBitmapInternals.hasInstance(value) ||
        OffscreenCanvasInternals.hasInstance(value))) ||
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
const colorSpaceConversionToRepr = ObjectFreeze({
  __proto__: null,
  "default": 0,
  "none": 1,
});
const resizeQualityToRepr = ObjectFreeze({
  __proto__: null,
  "pixelated": 0,
  "low": 1,
  "medium": 2,
  "high": 3,
});

async function checkUsabilityAndCropWithFormatting(
  image,
  sx,
  sy,
  sw,
  sh,
  dw,
  dh,
  resizeQuality,
  imageOrientation,
  colorSpaceConversion,
) {
  if (OffscreenCanvasInternals.hasInstance(image)) {
    const ctx = OffscreenCanvasInternals.getContext(image);
    if (!ctx) {
      throw new DOMException("Canvas is detached", "InvalidStateError");
    }
    const width = OffscreenCanvasInternals.getWidth(image);
    const height = OffscreenCanvasInternals.getHeight(image);
    if (width === 0 || height === 0) {
      throw new DOMException("Canvas has no pixels", "InvalidStateError");
    }
    try {
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
    } finally {
      await makeSafePromise(op_defer());
    }
  }
  if (ImageBitmapInternals.hasInstance(image)) {
    const raw = ImageBitmapInternals.getRaw(image);
    if (!raw) {
      throw new DOMException("Image is detached", "InvalidStateError");
    }
    try {
      return op_canvas_2d_image_bitmap_resize(
        op_canvas_2d_image_bitmap_crop(raw, sx, sy, sw ?? 0, sh ?? 0),
        dw ?? 0,
        dh ?? 0,
        resizeQualityToRepr[resizeQuality],
        imageOrientationToRepr[imageOrientation],
      );
    } finally {
      await makeSafePromise(op_defer());
    }
  }
  if (isBlob(image)) {
    try {
      return op_canvas_2d_decode_image(
        await makeSafePromise(BlobPrototypeBytes(image)),
        BlobPrototypeGetType(image),
        sx,
        sy,
        sw ?? 0,
        sh ?? 0,
        dw ?? 0,
        dh ?? 0,
        resizeQualityToRepr[resizeQuality],
        imageOrientationToRepr[imageOrientation],
        colorSpaceConversionToRepr[colorSpaceConversion],
      );
    } finally {
      await makeSafePromise(op_defer());
    }
  }
  const data = ImageDataPrototypeGetData(image);
  if (TypedArrayPrototypeGetLength(data) === 0) {
    throw new DOMException("Image data is detached", "InvalidStateError");
  }
  const width = ImageDataPrototypeGetWidth(image);
  const height = ImageDataPrototypeGetHeight(image);
  const colorSpace = ImageDataPrototypeGetColorSpace(image);
  try {
    return op_canvas_2d_image_bitmap_from_image_data_crop_and_resize(
      new Uint8Array(
        TypedArrayPrototypeGetBuffer(data),
        TypedArrayPrototypeGetByteOffset(data),
        TypedArrayPrototypeGetByteLength(data),
      ),
      width,
      height,
      colorSpaceToRepr[colorSpace],
      sx,
      sy,
      sw ?? 0,
      sh ?? 0,
      dw ?? 0,
      dh ?? 0,
      resizeQualityToRepr[resizeQuality],
      imageOrientationToRepr[imageOrientation],
    );
  } finally {
    await makeSafePromise(op_defer());
  }
}

async function createImageBitmapInner(image, sx, sy, sw, sh, options) {
  const {
    resizeWidth,
    resizeHeight,
    resizeQuality,
    imageOrientation,
    colorSpaceConversion,
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
  const bitmap = await makeSafePromise(checkUsabilityAndCropWithFormatting(
    image,
    sx,
    sy,
    sw,
    sh,
    resizeWidth,
    resizeHeight,
    resizeQuality,
    imageOrientation,
    colorSpaceConversion,
  ));
  return createImageBitmapFromRaw(bitmap);
}

export const makeCreateImageBitmap = (prefix) => ({
  createImageBitmap(image, sx = undefined, sy, sw, sh, options) {
    try {
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
      return createImageBitmapInner(image, sx, sy, sw, sh, options);
    } catch (e) {
      return PromiseReject(e);
    }
  },
}.createImageBitmap);
