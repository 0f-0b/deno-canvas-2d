import {
  op_canvas_2d_image_bitmap_clone,
  op_canvas_2d_image_bitmap_close,
  op_canvas_2d_image_bitmap_color_space,
  op_canvas_2d_image_bitmap_crop,
  op_canvas_2d_image_bitmap_empty,
  op_canvas_2d_image_bitmap_get_image_data,
  op_canvas_2d_image_bitmap_height,
  op_canvas_2d_image_bitmap_remove_alpha,
  op_canvas_2d_image_bitmap_width,
} from "ext:canvas_2d/00_ops.js";
import { createDictionaryConverter } from "ext:canvas_2d/04_create_dictionary_converter.js";
import { convertBoolean } from "ext:canvas_2d/05_convert_boolean.js";
import {
  colorSpaceFromRepr,
  colorSpaceToRepr,
  objectIsImageBitmap,
  registerCanvasContextMode,
  transferImageBitmap,
} from "ext:canvas_2d/16_canvas.js";
import { primordials } from "ext:core/mod.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import {
  configureInterface,
  illegalConstructor,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";

const {
  RangeError,
  SymbolFor,
  TypeError,
  TypedArrayPrototypeGetBuffer,
  Uint32Array,
  Uint8Array,
} = primordials;
const privateCustomInspect = SymbolFor("Deno.privateCustomInspect");
const convertNullableImageBitmap = (value) => {
  if (value === null || value === undefined) {
    return null;
  }
  if (type(value) === "Object" && objectIsImageBitmap(value)) {
    return value;
  }
  throw new TypeError("Expected ImageBitmap");
};
let objectIsImageBitmapRenderingContext;
let getImageBitmapRenderingContextBitmap;
let setImageBitmapRenderingContextBitmap;

export class ImageBitmapRenderingContext {
  #brand() {}

  #canvas;
  #bitmap;
  #alpha;

  constructor(key = undefined, target, width, height, settings) {
    if (key !== illegalConstructor) {
      illegalConstructor();
    }
    settings = convertImageBitmapRenderingContextSettings(settings);
    if (width > 0xffffffff) {
      throw new RangeError(`Invalid bitmap width: ${width}`);
    }
    if (height > 0xffffffff) {
      throw new RangeError(`Invalid bitmap height: ${height}`);
    }
    this.#canvas = target;
    this.#alpha = settings.alpha;
    this.#setBitmap(op_canvas_2d_image_bitmap_empty(width, height));
  }

  get canvas() {
    return this.#canvas;
  }

  #setBitmap(bitmap) {
    if (!this.#alpha) {
      bitmap = op_canvas_2d_image_bitmap_remove_alpha(bitmap);
    }
    this.#bitmap = bitmap;
  }

  transferFromImageBitmap(bitmap) {
    this.#brand;
    const prefix =
      "Failed to execute 'transferFromImageBitmap' on 'ImageBitmapRenderingContext'";
    requiredArguments(arguments.length, 1, prefix);
    bitmap = convertNullableImageBitmap(bitmap);
    if (!bitmap) {
      const raw = op_canvas_2d_image_bitmap_empty(width, height);
      op_canvas_2d_image_bitmap_close(this.#bitmap);
      this.#setBitmap(raw);
      return;
    }
    const raw = transferImageBitmap(bitmap);
    if (!raw) {
      throw new DOMException("Image is detached", "InvalidStateError");
    }
    op_canvas_2d_image_bitmap_close(this.#bitmap);
    this.#setBitmap(raw);
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["canvas"],
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
    objectIsImageBitmapRenderingContext = (o) => #brand in o;
    getImageBitmapRenderingContextBitmap = (o) => o.#bitmap;
    setImageBitmapRenderingContextBitmap = (o, v) => o.#setBitmap(v);
  }
}

const readImageBitmapRenderingContextSettingsMembers = (value) => {
  const result = { __proto__: null };
  const { alpha = true } = value;
  result.alpha = convertBoolean(alpha);
  return result;
};
const convertImageBitmapRenderingContextSettings = createDictionaryConverter(
  readImageBitmapRenderingContextSettingsMembers,
);
registerCanvasContextMode("bitmaprenderer", {
  newInstance(canvas, width, height, options) {
    return new ImageBitmapRenderingContext(
      illegalConstructor,
      canvas,
      width,
      height,
      options,
    );
  },
  hasInstance(ctx) {
    return objectIsImageBitmapRenderingContext(ctx);
  },
  getWidth(ctx) {
    const bitmap = getImageBitmapRenderingContextBitmap(ctx);
    return op_canvas_2d_image_bitmap_width(bitmap);
  },
  setWidth(ctx, width) {
    if (width > 0xffffffff) {
      throw new RangeError(`Invalid bitmap width: ${width}`);
    }
    const bitmap = getImageBitmapRenderingContextBitmap(ctx);
    const height = op_canvas_2d_image_bitmap_height(bitmap);
    op_canvas_2d_image_bitmap_close(bitmap);
    setImageBitmapRenderingContextBitmap(
      ctx,
      op_canvas_2d_image_bitmap_empty(width, height),
    );
  },
  getHeight(ctx) {
    const bitmap = getImageBitmapRenderingContextBitmap(ctx);
    return op_canvas_2d_image_bitmap_height(bitmap);
  },
  setHeight(ctx, height) {
    if (height > 0xffffffff) {
      throw new RangeError(`Invalid bitmap height: ${height}`);
    }
    const bitmap = getImageBitmapRenderingContextBitmap(ctx);
    const width = op_canvas_2d_image_bitmap_width(bitmap);
    op_canvas_2d_image_bitmap_close(bitmap);
    setImageBitmapRenderingContextBitmap(
      ctx,
      op_canvas_2d_image_bitmap_empty(width, height),
    );
  },
  transferToImageBitmap(ctx) {
    const bitmap = getImageBitmapRenderingContextBitmap(ctx);
    const width = op_canvas_2d_image_bitmap_width(bitmap);
    const height = op_canvas_2d_image_bitmap_height(bitmap);
    setImageBitmapRenderingContextBitmap(
      ctx,
      op_canvas_2d_image_bitmap_empty(width, height),
    );
    return bitmap;
  },
  cloneToImageBitmap(ctx) {
    const bitmap = getImageBitmapRenderingContextBitmap(ctx);
    return op_canvas_2d_image_bitmap_clone(bitmap);
  },
  cropToImageBitmap(ctx, sx, sy, sw, sh) {
    const bitmap = getImageBitmapRenderingContextBitmap(ctx);
    return {
      bitmap: op_canvas_2d_image_bitmap_crop(bitmap, sx, sy, sw ?? 0, sh ?? 0),
      needResize: true,
    };
  },
  getDataForSerialization(ctx, colorSpace) {
    const bitmap = getImageBitmapRenderingContextBitmap(ctx);
    colorSpace ??= colorSpaceFromRepr[
      op_canvas_2d_image_bitmap_color_space(bitmap)
    ];
    const width = op_canvas_2d_image_bitmap_width(bitmap);
    const height = op_canvas_2d_image_bitmap_height(bitmap);
    const buf = new Uint32Array(width * height);
    op_canvas_2d_image_bitmap_get_image_data(
      bitmap,
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
