import {
  ImageBitmap,
  makeCreateImageBitmap,
  OffscreenCanvas,
} from "ext:canvas_2d/16_canvas.js";
import {
  FontFace,
  FontFaceSet,
  FontFaceSetLoadEvent,
  getFonts,
} from "ext:canvas_2d/16_font_loading.js";
import {
  CanvasGradient,
  CanvasPattern,
  OffscreenCanvasRenderingContext2D,
  Path2D,
  TextMetrics,
} from "ext:canvas_2d/17_context_2d.js";
import { ImageBitmapRenderingContext } from "ext:canvas_2d/17_context_bitmaprenderer.js";
import { core, primordials } from "ext:core/mod.js";

const { ObjectDefineProperties, globalThis } = primordials;
const { loadExtScript, propGetterOnly, propNonEnumerable, propWritable } = core;
const { URL } = loadExtScript("ext:deno_web/00_url.js");
ObjectDefineProperties(globalThis, {
  URL: propNonEnumerable(URL),
  CanvasGradient: propNonEnumerable(CanvasGradient),
  CanvasPattern: propNonEnumerable(CanvasPattern),
  TextMetrics: propNonEnumerable(TextMetrics),
  Path2D: propNonEnumerable(Path2D),
  ImageBitmapRenderingContext: propNonEnumerable(ImageBitmapRenderingContext),
  OffscreenCanvas: propNonEnumerable(OffscreenCanvas),
  OffscreenCanvasRenderingContext2D: propNonEnumerable(
    OffscreenCanvasRenderingContext2D,
  ),
  ImageBitmap: propNonEnumerable(ImageBitmap),
  FontFace: propNonEnumerable(FontFace),
  FontFaceSetLoadEvent: propNonEnumerable(FontFaceSetLoadEvent),
  FontFaceSet: propNonEnumerable(FontFaceSet),
  createImageBitmap: propWritable(
    makeCreateImageBitmap("Failed to execute 'createImageBitmap' on 'Window'"),
  ),
  // Install `fonts` on `globalThis` since we don't have a `Document`
  fonts: propGetterOnly(getFonts),
});
