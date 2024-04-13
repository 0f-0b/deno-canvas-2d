import { primordials } from "ext:core/mod.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import {
  configureInterface,
  illegalConstructor,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";
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
} from "./00_ops.js";
import { IdentityConstructor } from "./01_identity_constructor.js";
import { createDictionaryConverter } from "./04_create_dictionary_converter.js";
import { convertBoolean } from "./05_convert_boolean.js";
import {
  colorSpaceFromRepr,
  colorSpaceToRepr,
  ImageBitmapInternals,
  registerCanvasContextMode,
} from "./16_canvas.js";

const {
  Object,
  ObjectCreate,
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
  if (type(value) === "Object" && ImageBitmapInternals.hasInstance(value)) {
    return value;
  }
  throw new TypeError("Expected ImageBitmap");
};
const ImageBitmapRenderingContextInternals = class ImageBitmapRenderingContext
  extends IdentityConstructor {
  #brand() {}

  #canvas;
  #bitmap;
  #alpha;

  constructor(o, canvas, bitmap, alpha) {
    super(o);
    this.#canvas = canvas;
    this.#alpha = alpha;
    this.#setBitmap(bitmap);
  }

  #setBitmap(bitmap) {
    if (!this.#alpha) {
      bitmap = op_canvas_2d_image_bitmap_remove_alpha(bitmap);
    }
    this.#bitmap = bitmap;
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

  static getBitmap(o) {
    return o.#bitmap;
  }

  static setBitmap(o, bitmap) {
    o.#setBitmap(bitmap);
  }

  static inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: ["canvas"],
      }),
      options,
    );
  }
};

export class ImageBitmapRenderingContext extends Object {
  // deno-lint-ignore constructor-super
  constructor() {
    illegalConstructor();
  }

  get canvas() {
    return ImageBitmapRenderingContextInternals.getCanvas(this);
  }

  transferFromImageBitmap(bitmap) {
    ImageBitmapRenderingContextInternals.checkInstance(this);
    const prefix =
      "Failed to execute 'transferFromImageBitmap' on 'ImageBitmapRenderingContext'";
    requiredArguments(arguments.length, 1, prefix);
    bitmap = convertNullableImageBitmap(bitmap);
    if (!bitmap) {
      const raw = op_canvas_2d_image_bitmap_empty(width, height);
      op_canvas_2d_image_bitmap_close(
        ImageBitmapRenderingContextInternals.getBitmap(this),
      );
      ImageBitmapRenderingContextInternals.setBitmap(this, raw);
      return;
    }
    const raw = ImageBitmapInternals.transfer(bitmap);
    if (!raw) {
      throw new DOMException("Image is detached", "InvalidStateError");
    }
    op_canvas_2d_image_bitmap_close(
      ImageBitmapRenderingContextInternals.getBitmap(this),
    );
    ImageBitmapRenderingContextInternals.setBitmap(this, raw);
  }

  get [privateCustomInspect]() {
    return ImageBitmapRenderingContextInternals.hasInstance(this)
      ? ImageBitmapRenderingContextInternals.inspect
      : undefined;
  }

  static {
    configureInterface(this);
  }
}

function createImageBitmapRenderingContext(canvas, bitmap, alpha) {
  const o = ObjectCreate(ImageBitmapRenderingContext.prototype);
  new ImageBitmapRenderingContextInternals(o, canvas, bitmap, alpha);
  return o;
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
    const settings = convertImageBitmapRenderingContextSettings(options);
    if (width > 0xffffffff) {
      throw new RangeError(`Invalid bitmap width: ${width}`);
    }
    if (height > 0xffffffff) {
      throw new RangeError(`Invalid bitmap height: ${height}`);
    }
    return createImageBitmapRenderingContext(
      canvas,
      op_canvas_2d_image_bitmap_empty(width, height),
      settings.alpha,
    );
  },
  hasInstance(ctx) {
    return ImageBitmapRenderingContextInternals.hasInstance(ctx);
  },
  getWidth(ctx) {
    const bitmap = ImageBitmapRenderingContextInternals.getBitmap(ctx);
    return op_canvas_2d_image_bitmap_width(bitmap);
  },
  setWidth(ctx, width) {
    if (width > 0xffffffff) {
      throw new RangeError(`Invalid bitmap width: ${width}`);
    }
    const bitmap = ImageBitmapRenderingContextInternals.getBitmap(ctx);
    const height = op_canvas_2d_image_bitmap_height(bitmap);
    op_canvas_2d_image_bitmap_close(bitmap);
    ImageBitmapRenderingContextInternals.setBitmap(
      ctx,
      op_canvas_2d_image_bitmap_empty(width, height),
    );
  },
  getHeight(ctx) {
    const bitmap = ImageBitmapRenderingContextInternals.getBitmap(ctx);
    return op_canvas_2d_image_bitmap_height(bitmap);
  },
  setHeight(ctx, height) {
    if (height > 0xffffffff) {
      throw new RangeError(`Invalid bitmap height: ${height}`);
    }
    const bitmap = ImageBitmapRenderingContextInternals.getBitmap(ctx);
    const width = op_canvas_2d_image_bitmap_width(bitmap);
    op_canvas_2d_image_bitmap_close(bitmap);
    ImageBitmapRenderingContextInternals.setBitmap(
      ctx,
      op_canvas_2d_image_bitmap_empty(width, height),
    );
  },
  transferToImageBitmap(ctx) {
    const bitmap = ImageBitmapRenderingContextInternals.getBitmap(ctx);
    const width = op_canvas_2d_image_bitmap_width(bitmap);
    const height = op_canvas_2d_image_bitmap_height(bitmap);
    ImageBitmapRenderingContextInternals.setBitmap(
      ctx,
      op_canvas_2d_image_bitmap_empty(width, height),
    );
    return bitmap;
  },
  cloneToImageBitmap(ctx) {
    const bitmap = ImageBitmapRenderingContextInternals.getBitmap(ctx);
    return op_canvas_2d_image_bitmap_clone(bitmap);
  },
  cropToImageBitmap(ctx, sx, sy, sw, sh) {
    const bitmap = ImageBitmapRenderingContextInternals.getBitmap(ctx);
    return {
      bitmap: op_canvas_2d_image_bitmap_crop(bitmap, sx, sy, sw ?? 0, sh ?? 0),
      needResize: true,
    };
  },
  getDataForSerialization(ctx, colorSpace) {
    const bitmap = ImageBitmapRenderingContextInternals.getBitmap(ctx);
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
