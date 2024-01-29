import { primordials } from "ext:core/mod.js";
import {
  op_canvas_2d_font_face_errored,
  op_canvas_2d_font_face_family,
  op_canvas_2d_font_face_load_binary_data,
  op_canvas_2d_font_face_new,
  op_canvas_2d_font_face_set_add,
  op_canvas_2d_font_face_set_family,
  op_canvas_2d_font_face_set_new,
  op_canvas_2d_font_face_set_unicode_range,
  op_canvas_2d_font_face_unicode_range,
  op_canvas_2d_font_source,
} from "ext:deno_canvas_2d/00_ops.js";
import { IdentityConstructor } from "ext:deno_canvas_2d/01_identity_constructor.js";
import { requireObject } from "ext:deno_canvas_2d/01_require_object.js";
import { isArrayBuffer } from "ext:deno_canvas_2d/02_is_array_buffer.js";
import { isDataView } from "ext:deno_canvas_2d/02_is_data_view.js";
import { isTypedArray } from "ext:deno_canvas_2d/02_is_typed_array.js";
import { createDictionaryConverter } from "ext:deno_canvas_2d/04_create_dictionary_converter.js";
import { createSequenceFromIterable } from "ext:deno_canvas_2d/04_create_sequence_from_iterable.js";
import { convertArrayBuffer } from "ext:deno_canvas_2d/05_convert_array_buffer.js";
import { convertDataView } from "ext:deno_canvas_2d/05_convert_data_view.js";
import { convertDOMString } from "ext:deno_canvas_2d/05_convert_dom_string.js";
import { convertEventHandler } from "ext:deno_canvas_2d/05_convert_event_handler.js";
import { convertTypedArray } from "ext:deno_canvas_2d/05_convert_typed_array.js";
import {
  EventHandler,
  readEventInitMembers,
} from "ext:deno_canvas_2d/15_event.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import {
  dispatch,
  Event,
  EventTarget,
  setIsTrusted,
} from "ext:deno_web/02_event.js";
import { defer } from "ext:deno_web/02_timers.js";
import {
  configureInterface,
  illegalConstructor,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";

const {
  ObjectFreeze,
  Promise,
  ReflectConstruct,
  SafeArrayIterator,
  SafeSet,
  SymbolFor,
  SymbolIterator,
  SymbolToStringTag,
  TypeError,
  globalThis,
} = primordials;
const privateCustomInspect = SymbolFor("Deno.privateCustomInspect");
const empty = ObjectFreeze({
  next: () => ({ done: true }),
  [SymbolIterator]: () => empty,
});
const safePromiseWithResolvers = () => {
  let resolve;
  let reject;
  const promise = new Promise((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return { promise, resolve, reject };
};
const convertDOMStringOrBinaryData = (value) => {
  if (isArrayBuffer(value)) {
    return convertArrayBuffer(value);
  }
  if (isDataView(value)) {
    return convertDataView(value);
  }
  if (isTypedArray(value)) {
    return convertTypedArray(value);
  }
  return convertDOMString(value);
};
const readFontFaceDescriptorsMembers = (value) => {
  const result = { __proto__: null };
  const { style = "normal" } = value;
  result.style = convertDOMString(style);
  const { weight = "normal" } = value;
  result.weight = convertDOMString(weight);
  const { stretch = "normal" } = value;
  result.stretch = convertDOMString(stretch);
  const { unicodeRange = "U+0-10FFFF" } = value;
  result.unicodeRange = convertDOMString(unicodeRange);
  const { featureSettings = "normal" } = value;
  result.featureSettings = convertDOMString(featureSettings);
  const { variationSettings = "normal" } = value;
  result.variationSettings = convertDOMString(variationSettings);
  const { display = "auto" } = value;
  result.display = convertDOMString(display);
  const { ascentOverride = "normal" } = value;
  result.ascentOverride = convertDOMString(ascentOverride);
  const { descentOverride = "normal" } = value;
  result.descentOverride = convertDOMString(descentOverride);
  const { lineGapOverride = "normal" } = value;
  result.lineGapOverride = convertDOMString(lineGapOverride);
  return result;
};
const convertFontFaceDescriptors = createDictionaryConverter(
  readFontFaceDescriptorsMembers,
);
let objectIsFontFace;
let getFontFaceRaw;
let getFontFaceStatus;
let addFontFaceToSet;

export class FontFace {
  #brand() {}

  #raw;
  #status = "unloaded";
  #loaded = safePromiseWithResolvers();
  #fontFaceSets = new SafeSet();
  #cachedFamily = null;
  #cachedUnicodeRange = null;

  constructor(family, source, descriptors = undefined) {
    family = convertDOMString(family);
    source = convertDOMStringOrBinaryData(source);
    descriptors = convertFontFaceDescriptors(descriptors);
    try {
      this.#raw = op_canvas_2d_font_face_new(
        family,
        descriptors.style,
        descriptors.weight,
        descriptors.stretch,
        descriptors.unicodeRange,
        descriptors.featureSettings,
        descriptors.variationSettings,
        descriptors.display,
        descriptors.ascentOverride,
        descriptors.descentOverride,
        descriptors.lineGapOverride,
        typeof source === "string" ? source : null,
      );
    } catch (e) {
      this.#setError(e);
      this.#raw = op_canvas_2d_font_face_errored();
      return;
    }
    if (typeof source === "string") {
      return;
    }
    defer(() => {
      this.#setLoading();
      try {
        op_canvas_2d_font_face_load_binary_data(this.#raw, source);
      } catch (e) {
        defer(() => this.#setError(e));
        return;
      }
      defer(() => this.#setLoaded());
    });
  }

  get family() {
    this.#brand;
    this.#cachedFamily ??= op_canvas_2d_font_face_family(this.#raw);
    return this.#cachedFamily;
  }

  set family(value) {
    this.#brand;
    const prefix = "Failed to set 'family' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_family(this.#raw, value);
    this.#cachedFamily = null;
  }

  get style() {
    this.#brand;
    return "normal"; // TODO implement
  }

  set style(value) {
    this.#brand;
    const prefix = "Failed to set 'style' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get weight() {
    this.#brand;
    return "normal"; // TODO implement
  }

  set weight(value) {
    this.#brand;
    const prefix = "Failed to set 'weight' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get stretch() {
    this.#brand;
    return "normal"; // TODO implement
  }

  set stretch(value) {
    this.#brand;
    const prefix = "Failed to set 'stretch' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get unicodeRange() {
    this.#brand;
    this.#cachedUnicodeRange ??= op_canvas_2d_font_face_unicode_range(
      this.#raw,
    );
    return this.#cachedUnicodeRange;
  }

  set unicodeRange(value) {
    this.#brand;
    const prefix = "Failed to set 'unicodeRange' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_unicode_range(this.#raw, value);
    this.#cachedUnicodeRange = null;
  }

  get featureSettings() {
    this.#brand;
    return "normal"; // TODO implement
  }

  set featureSettings(value) {
    this.#brand;
    const prefix = "Failed to set 'featureSettings' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get variationSettings() {
    this.#brand;
    return "normal"; // TODO implement
  }

  set variationSettings(value) {
    this.#brand;
    const prefix = "Failed to set 'variationSettings' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get display() {
    this.#brand;
    return "auto"; // TODO implement
  }

  set display(value) {
    this.#brand;
    const prefix = "Failed to set 'display' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get ascentOverride() {
    this.#brand;
    return "normal"; // TODO implement
  }

  set ascentOverride(value) {
    this.#brand;
    const prefix = "Failed to set 'ascentOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get descentOverride() {
    this.#brand;
    return "normal"; // TODO implement
  }

  set descentOverride(value) {
    this.#brand;
    const prefix = "Failed to set 'descentOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get lineGapOverride() {
    this.#brand;
    return "normal"; // TODO implement
  }

  set lineGapOverride(value) {
    this.#brand;
    const prefix = "Failed to set 'lineGapOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  #setLoading() {
    this.#status = "loading";
    // deno-lint-ignore prefer-primordials
    for (const set of this.#fontFaceSets) {
      FontFaceSetInternals.addLoading(set, this);
    }
  }

  #setLoaded() {
    this.#loaded.resolve(this);
    this.#status = "loaded";
    // deno-lint-ignore prefer-primordials
    for (const set of this.#fontFaceSets) {
      FontFaceSetInternals.addLoaded(set, this);
    }
    this.#fontFaceSets = null;
  }

  #setError(reason) {
    this.#loaded.reject(reason);
    this.#status = "error";
    // deno-lint-ignore prefer-primordials
    for (const set of this.#fontFaceSets) {
      FontFaceSetInternals.addFailed(set, this);
    }
    this.#fontFaceSets = null;
  }

  get status() {
    return this.#status;
  }

  load() {
    this.#brand;
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get loaded() {
    return this.#loaded.promise;
  }

  #inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: [
          "family",
          "style",
          "weight",
          "stretch",
          "unicodeRange",
          "featureSettings",
          "variationSettings",
          "display",
          "ascentOverride",
          "descentOverride",
          "lineGapOverride",
          "status",
          "loaded",
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
    objectIsFontFace = (o) => #brand in o;
    getFontFaceRaw = (o) => o.#raw;
    getFontFaceStatus = (o) => o.#status;
    addFontFaceToSet = (o, set) => void o.#fontFaceSets?.add(set);
  }
}

const convertFontFace = (value) => {
  if (!(type(value) === "Object" && objectIsFontFace(value))) {
    throw new TypeError("Expected FontFace");
  }
  return value;
};
const convertSequenceOfFontFace = (value) => {
  const method = requireObject(value)[SymbolIterator];
  return createSequenceFromIterable(value, method, convertFontFace);
};
const readFontFaceSetLoadEventInitMembers = (value) => {
  const result = readEventInitMembers(value);
  const { fontfaces = empty } = value;
  result.fontfaces = convertSequenceOfFontFace(fontfaces);
  return result;
};
const convertFontFaceSetLoadEventInit = createDictionaryConverter(
  readFontFaceSetLoadEventInitMembers,
);

export const FontFaceSetLoadEventInternals = class FontFaceSetLoadEvent
  extends IdentityConstructor {
  #brand() {}

  #fontfaces;

  constructor(o, fontfaces) {
    super(o);
    this[SymbolToStringTag] = "FontFaceSetLoadEvent";
    this.#fontfaces = fontfaces;
  }

  static hasInstance(o) {
    // deno-lint-ignore prefer-primordials
    return #brand in o;
  }

  static checkInstance(o) {
    o.#brand;
  }

  static getFontFaces(o) {
    return o.#fontfaces;
  }

  static inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: [
          "type",
          "target",
          "srcElement",
          "currentTarget",
          "eventPhase",
          "cancelBubble",
          "bubbles",
          "cancelable",
          "returnValue",
          "defaultPrevented",
          "composed",
          "isTrusted",
          "timeStamp",
          "fontfaces",
        ],
      }),
      options,
    );
  }
};

export class FontFaceSetLoadEvent extends Event {
  constructor(type, eventInitDict = undefined) {
    type = convertDOMString(type);
    eventInitDict = convertFontFaceSetLoadEventInit(eventInitDict);
    return new FontFaceSetLoadEventInternals(
      ReflectConstruct(Event, [type, eventInitDict], new.target),
      ObjectFreeze(eventInitDict.fontfaces),
    );
  }

  get fontfaces() {
    return FontFaceSetLoadEventInternals.getFontFaces(this);
  }

  get [privateCustomInspect]() {
    return FontFaceSetLoadEventInternals.hasInstance(this)
      ? FontFaceSetLoadEventInternals.inspect
      : undefined;
  }

  static {
    configureInterface(this);
    delete this.prototype[SymbolToStringTag];
  }
}

export const FontFaceSetInternals = class FontFaceSet
  extends IdentityConstructor {
  #brand() {}

  #raw;
  #entries = new SafeSet();
  #status = "loaded";
  #ready = safePromiseWithResolvers();
  #readyResolved = false;
  #loadingFonts = new SafeSet();
  #loadedFonts = new SafeSet();
  #failedFonts = new SafeSet();
  #onloading = new EventHandler(this, "loading");
  #onloadingdone = new EventHandler(this, "loadingdone");
  #onloadingerror = new EventHandler(this, "loadingerror");

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

  static getStatus(o) {
    return o.#status;
  }

  static getReady(o) {
    return o.#ready.promise;
  }

  static getOnLoading(o) {
    return o.#onloading;
  }

  static getOnLoadingDone(o) {
    return o.#onloadingdone;
  }

  static getOnLoadingError(o) {
    return o.#onloadingerror;
  }

  static add(o, font) {
    if (!o.#entries.has(font)) {
      o.#entries.add(font);
      op_canvas_2d_font_face_set_add(o.#raw, getFontFaceRaw(font));
      addFontFaceToSet(font, o);
      if (getFontFaceStatus(font) === "loading") {
        FontFaceSetInternals.addLoading(o, font);
      }
    }
  }

  static addLoading(o, font) {
    if (o.#loadingFonts.size === 0) {
      FontFaceSetInternals.switchToLoading(o);
    }
    o.#loadingFonts.add(font);
  }

  static addLoaded(o, font) {
    o.#loadedFonts.add(font);
    o.#loadingFonts.delete(font);
    if (o.#loadingFonts.size === 0) {
      FontFaceSetInternals.switchToLoaded(o);
    }
  }

  static addFailed(o, font) {
    o.#failedFonts.add(font);
    o.#loadingFonts.delete(font);
    if (o.#loadingFonts.size === 0) {
      FontFaceSetInternals.switchToLoaded(o);
    }
  }

  static fireFontLoadEvent(o, e, fontfaces) {
    const event = new FontFaceSetLoadEvent(e, { __proto__: null, fontfaces });
    setIsTrusted(event, true);
    dispatch(o, event);
  }

  static switchToLoading(o) {
    o.#status = "loading";
    if (o.#readyResolved) {
      o.#ready = safePromiseWithResolvers();
      o.#readyResolved = false;
    }
    defer(() => FontFaceSetInternals.fireFontLoadEvent(o, "loading"));
  }

  static switchToLoaded(o) {
    o.#status = "loaded";
    o.#ready.resolve(o);
    defer(() => {
      const loadedFonts = o.#loadedFonts;
      const failedFonts = o.#failedFonts;
      o.#loadedFonts = new SafeSet();
      o.#failedFonts = new SafeSet();
      FontFaceSetInternals.fireFontLoadEvent(o, "loadingdone", loadedFonts);
      if (failedFonts.size !== 0) {
        FontFaceSetInternals.fireFontLoadEvent(o, "loadingerror", failedFonts);
      }
    });
  }

  static inspect(inspect, options) {
    return inspect(
      createFilteredInspectProxy({
        object: this,
        evaluate: true,
        keys: [
          "onloading",
          "onloadingdone",
          "onloadingerror",
          "ready",
          "status",
        ],
      }),
      options,
    );
  }
};

export class FontFaceSet extends EventTarget {
  constructor(initialFaces) {
    if (initialFaces === illegalConstructor) {
      return new FontFaceSetInternals(
        ReflectConstruct(EventTarget, [], new.target),
        op_canvas_2d_font_source(),
      );
    }
    initialFaces = convertSequenceOfFontFace(initialFaces);
    const o = new FontFaceSetInternals(
      ReflectConstruct(EventTarget, [], new.target),
      op_canvas_2d_font_face_set_new(),
    );
    for (const font of new SafeArrayIterator(initialFaces)) {
      FontFaceSetInternals.add(o, font);
    }
    return o;
  }

  // TODO setlike<FontFace>;

  add(font) {
    FontFaceSetInternals.checkInstance(this);
    const prefix = "Failed to execute 'add' on 'FontFaceSet'";
    requiredArguments(arguments.length, 1, prefix);
    font = convertFontFace(font);
    FontFaceSetInternals.add(this, font);
    return this;
  }

  delete(font) {
    FontFaceSetInternals.checkInstance(this);
    const prefix = "Failed to execute 'delete' on 'FontFaceSet'";
    requiredArguments(arguments.length, 1, prefix);
    font = convertFontFace(font);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  clear() {
    FontFaceSetInternals.checkInstance(this);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get onloading() {
    return FontFaceSetInternals.getOnLoading(this).value;
  }

  set onloading(value) {
    FontFaceSetInternals.checkInstance(this);
    const prefix = "Failed to set 'onloading' on 'FontFaceSet'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertEventHandler(value);
    FontFaceSetInternals.getOnLoading(this).update(value);
  }

  get onloadingdone() {
    return FontFaceSetInternals.getOnLoadingDone(this).value;
  }

  set onloadingdone(value) {
    FontFaceSetInternals.checkInstance(this);
    const prefix = "Failed to set 'onloadingdone' on 'FontFaceSet'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertEventHandler(value);
    FontFaceSetInternals.getOnLoadingDone(this).update(value);
  }

  get onloadingerror() {
    return FontFaceSetInternals.getOnLoadingError(this).value;
  }

  set onloadingerror(value) {
    FontFaceSetInternals.checkInstance(this);
    const prefix = "Failed to set 'onloadingerror' on 'FontFaceSet'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertEventHandler(value);
    FontFaceSetInternals.getOnLoadingError(this).update(value);
  }

  load(font, text = " ") {
    FontFaceSetInternals.checkInstance(this);
    const prefix = "Failed to execute 'load' on 'FontFaceSet'";
    requiredArguments(arguments.length, 1, prefix);
    font = convertDOMString(font);
    text = convertDOMString(text);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  check(font, text = " ") {
    FontFaceSetInternals.checkInstance(this);
    const prefix = "Failed to execute 'check' on 'FontFaceSet'";
    requiredArguments(arguments.length, 1, prefix);
    font = convertDOMString(font);
    text = convertDOMString(text);
    throw new TypeError("Unimplemented"); // TODO implement
  }

  get ready() {
    return FontFaceSetInternals.getReady(this);
  }

  get status() {
    return FontFaceSetInternals.getStatus(this);
  }

  get [privateCustomInspect]() {
    return FontFaceSetInternals.hasInstance(this)
      ? FontFaceSetInternals.inspect
      : undefined;
  }

  static {
    configureInterface(this);
  }
}

export const fonts = new FontFaceSet(illegalConstructor);
export const getFonts = ({
  "get fonts"() {
    if (this !== null && this !== undefined && this !== globalThis) {
      throw new TypeError("Illegal invocation");
    }
    return fonts;
  },
})["get fonts"];
