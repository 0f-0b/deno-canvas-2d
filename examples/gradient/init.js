import {
  DOMMatrix,
  DOMMatrixReadOnly,
  DOMPoint,
  DOMPointReadOnly,
  DOMQuad,
  DOMRect,
  DOMRectReadOnly,
} from "ext:canvas_2d/15_geometry.js";
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
import { URL } from "ext:deno_url/00_url.js";

const { ObjectDefineProperties, globalThis } = primordials;
const { propGetterOnly, propNonEnumerable, propWritable } = core;
ObjectDefineProperties(globalThis, {
  URL: propNonEnumerable(URL),
  DOMPointReadOnly: propNonEnumerable(DOMPointReadOnly),
  DOMPoint: propNonEnumerable(DOMPoint),
  DOMRectReadOnly: propNonEnumerable(DOMRectReadOnly),
  DOMRect: propNonEnumerable(DOMRect),
  DOMQuad: propNonEnumerable(DOMQuad),
  DOMMatrixReadOnly: propNonEnumerable(DOMMatrixReadOnly),
  DOMMatrix: propNonEnumerable(DOMMatrix),
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

// Suppress "following modules were not evaluated" warning
import "ext:deno_url/01_urlpattern.js";
import "ext:deno_web/04_global_interfaces.js";
import "ext:deno_web/05_base64.js";
import "ext:deno_web/10_filereader.js";
import "ext:deno_web/12_location.js";
import "ext:deno_web/13_message_port.js";
import "ext:deno_web/14_compression.js";
import "ext:deno_web/15_performance.js";
