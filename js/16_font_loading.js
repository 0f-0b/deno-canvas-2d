import { primordials } from "ext:core/mod.js";
import { createFilteredInspectProxy } from "ext:deno_console/01_console.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import {
  dispatch,
  Event,
  EventTarget,
  setIsTrusted,
} from "ext:deno_web/02_event.js";
import { defer } from "ext:deno_web/02_timers.js";
import { Deferred } from "ext:deno_web/06_streams.js";
import {
  configureInterface,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";
import {
  op_canvas_2d_font_face_ascent_override,
  op_canvas_2d_font_face_descent_override,
  op_canvas_2d_font_face_display,
  op_canvas_2d_font_face_errored,
  op_canvas_2d_font_face_family,
  op_canvas_2d_font_face_feature_settings,
  op_canvas_2d_font_face_id,
  op_canvas_2d_font_face_line_gap_override,
  op_canvas_2d_font_face_load,
  op_canvas_2d_font_face_new,
  op_canvas_2d_font_face_select_source,
  op_canvas_2d_font_face_set_ascent_override,
  op_canvas_2d_font_face_set_clear,
  op_canvas_2d_font_face_set_descent_override,
  op_canvas_2d_font_face_set_display,
  op_canvas_2d_font_face_set_family,
  op_canvas_2d_font_face_set_feature_settings,
  op_canvas_2d_font_face_set_insert,
  op_canvas_2d_font_face_set_line_gap_override,
  op_canvas_2d_font_face_set_match,
  op_canvas_2d_font_face_set_remove,
  op_canvas_2d_font_face_set_stretch,
  op_canvas_2d_font_face_set_style,
  op_canvas_2d_font_face_set_unicode_range,
  op_canvas_2d_font_face_set_variation_settings,
  op_canvas_2d_font_face_set_weight,
  op_canvas_2d_font_face_stretch,
  op_canvas_2d_font_face_style,
  op_canvas_2d_font_face_unicode_range,
  op_canvas_2d_font_face_variation_settings,
  op_canvas_2d_font_face_weight,
  op_canvas_2d_font_source,
} from "./00_ops.js";
import { capturePrototype } from "./01_capture_prototype.js";
import { IdentityConstructor } from "./01_identity_constructor.js";
import {
  makeSpeciesSafePromise,
  newFromSpeciesSafePromise,
  safePromiseAll,
} from "./01_promise.js";
import { requireObject } from "./01_require_object.js";
import { isArrayBuffer } from "./02_is_array_buffer.js";
import { isDataView } from "./02_is_data_view.js";
import { isTypedArray } from "./02_is_typed_array.js";
import { createDictionaryConverter } from "./04_create_dictionary_converter.js";
import { createSequenceFromIterable } from "./04_create_sequence_from_iterable.js";
import { convertArrayBuffer } from "./05_convert_array_buffer.js";
import { convertDataView } from "./05_convert_data_view.js";
import { convertDOMString } from "./05_convert_dom_string.js";
import { convertEventHandler } from "./05_convert_event_handler.js";
import { convertTypedArray } from "./05_convert_typed_array.js";
import { EventHandler, readEventInitMembers } from "./15_event.js";

const {
  ArrayPrototypeEvery,
  ArrayPrototypePush,
  FunctionPrototype,
  Object,
  ObjectCreate,
  ObjectDefineProperties,
  ObjectFreeze,
  ObjectGetOwnPropertyDescriptors,
  ObjectSetPrototypeOf,
  PromiseReject,
  ReflectConstruct,
  SafeArrayIterator,
  SafeSet,
  Set,
  SetPrototypeEntries,
  SetPrototypeForEach,
  SetPrototypeGetSize,
  SetPrototypeHas,
  SetPrototypeValues,
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
const convertDOMStringOrBufferSource = (value) => {
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
const FontFaceInternals = class FontFace extends IdentityConstructor {
  #brand() {}

  #raw;
  #url;
  #status = "unloaded";
  #innerLoaded = new Deferred();
  #loaded = newFromSpeciesSafePromise(
    makeSpeciesSafePromise(this.#innerLoaded.promise),
  );
  #fontFaceSets = new SafeSet();
  #cachedFamily = null;
  #cachedStyle = null;
  #cachedWeight = null;
  #cachedStretch = null;
  #cachedUnicodeRange = null;
  #cachedFeatureSettings = null;
  #cachedVariationSettings = null;
  #cachedDisplay = null;
  #cachedAscentOverride = null;
  #cachedDescentOverride = null;
  #cachedLineGapOverride = null;

  constructor(
    o,
    source,
    family,
    style,
    weight,
    stretch,
    unicodeRange,
    featureSettings,
    variationSettings,
    display,
    ascentOverride,
    descentOverride,
    lineGapOverride,
  ) {
    super(o);
    try {
      this.#url = typeof source === "string"
        ? op_canvas_2d_font_face_select_source(source)
        : null;
      this.#raw = op_canvas_2d_font_face_new(
        family,
        style,
        weight,
        stretch,
        unicodeRange,
        featureSettings,
        variationSettings,
        display,
        ascentOverride,
        descentOverride,
        lineGapOverride,
      );
    } catch (e) {
      FontFaceInternals.setError(this, e);
      this.#url = null;
      this.#raw = op_canvas_2d_font_face_errored();
      return;
    }
    if (typeof source === "string") {
      return;
    }
    defer(() => {
      FontFaceInternals.setLoading(this);
      defer(() => {
        try {
          op_canvas_2d_font_face_load(this.#raw, source, false);
        } catch (e) {
          FontFaceInternals.setError(this, e);
          return;
        }
        FontFaceInternals.setLoaded(this);
      });
    });
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

  static getStatus(o) {
    return o.#status;
  }

  static getLoaded(o) {
    return o.#loaded;
  }

  static addToSet(o, set) {
    o.#fontFaceSets?.add(set);
  }

  static getFamily(o) {
    o.#cachedFamily ??= op_canvas_2d_font_face_family(o.#raw);
    return o.#cachedFamily;
  }

  static setFamily(o, value) {
    op_canvas_2d_font_face_set_family(o.#raw, value);
    o.#cachedFamily = null;
  }

  static getStyle(o) {
    o.#cachedStyle ??= op_canvas_2d_font_face_style(o.#raw);
    return o.#cachedStyle;
  }

  static setStyle(o, value) {
    op_canvas_2d_font_face_set_style(o.#raw, value);
    o.#cachedStyle = null;
  }

  static getWeight(o) {
    o.#cachedWeight ??= op_canvas_2d_font_face_weight(o.#raw);
    return o.#cachedWeight;
  }

  static setWeight(o, value) {
    op_canvas_2d_font_face_set_weight(o.#raw, value);
    o.#cachedWeight = null;
  }

  static getStretch(o) {
    o.#cachedStretch ??= op_canvas_2d_font_face_stretch(o.#raw);
    return o.#cachedStretch;
  }

  static setStretch(o, value) {
    op_canvas_2d_font_face_set_stretch(o.#raw, value);
    o.#cachedStretch = null;
  }

  static getUnicodeRange(o) {
    o.#cachedUnicodeRange ??= op_canvas_2d_font_face_unicode_range(o.#raw);
    return o.#cachedUnicodeRange;
  }

  static setUnicodeRange(o, value) {
    op_canvas_2d_font_face_set_unicode_range(o.#raw, value);
    o.#cachedUnicodeRange = null;
  }

  static getFeatureSettings(o) {
    o.#cachedFeatureSettings ??= op_canvas_2d_font_face_feature_settings(
      o.#raw,
    );
    return o.#cachedFeatureSettings;
  }

  static setFeatureSettings(o, value) {
    op_canvas_2d_font_face_set_feature_settings(o.#raw, value);
    o.#cachedFeatureSettings = null;
  }

  static getVariationSettings(o) {
    o.#cachedVariationSettings = op_canvas_2d_font_face_variation_settings(
      o.#raw,
    );
    return o.#cachedVariationSettings;
  }

  static setVariationSettings(o, value) {
    op_canvas_2d_font_face_set_variation_settings(o.#raw, value);
    o.#cachedVariationSettings = null;
  }

  static getDisplay(o) {
    o.#cachedDisplay = op_canvas_2d_font_face_display(o.#raw);
    return o.#cachedDisplay;
  }

  static setDisplay(o, value) {
    op_canvas_2d_font_face_set_display(o.#raw, value);
    o.#cachedDisplay = null;
  }

  static getAscentOverride(o) {
    o.#cachedAscentOverride = op_canvas_2d_font_face_ascent_override(o.#raw);
    return o.#cachedAscentOverride;
  }

  static setAscentOverride(o, value) {
    op_canvas_2d_font_face_set_ascent_override(o.#raw, value);
    o.#cachedAscentOverride = null;
  }

  static getDescentOverride(o) {
    o.#cachedDescentOverride = op_canvas_2d_font_face_descent_override(o.#raw);
    return o.#cachedDescentOverride;
  }

  static setDescentOverride(o, value) {
    op_canvas_2d_font_face_set_descent_override(o.#raw, value);
    o.#cachedDescentOverride = null;
  }

  static getLineGapOverride(o) {
    o.#cachedLineGapOverride = op_canvas_2d_font_face_line_gap_override(o.#raw);
    return o.#cachedLineGapOverride;
  }

  static setLineGapOverride(o, value) {
    op_canvas_2d_font_face_set_line_gap_override(o.#raw, value);
    o.#cachedLineGapOverride = null;
  }

  static setLoading(o) {
    o.#status = "loading";
    // deno-lint-ignore prefer-primordials
    for (const set of o.#fontFaceSets) {
      FontFaceSetInternals.addLoading(set, o);
    }
  }

  static setLoaded(o) {
    o.#innerLoaded.resolve(o);
    o.#status = "loaded";
    // deno-lint-ignore prefer-primordials
    for (const set of o.#fontFaceSets) {
      FontFaceSetInternals.addLoaded(set, o);
    }
    o.#fontFaceSets = null;
  }

  static setError(o, reason) {
    o.#innerLoaded.reject(reason);
    o.#status = "error";
    // deno-lint-ignore prefer-primordials
    for (const set of o.#fontFaceSets) {
      FontFaceSetInternals.addFailed(set, o);
    }
    o.#fontFaceSets = null;
  }

  static load(o) {
    const url = o.#url;
    if (url !== null) {
      o.#url = null;
      FontFaceInternals.setLoading(o);
      fetchFont(url, (result) => {
        if (!result.success) {
          FontFaceInternals.setError(o, result.reason);
          return;
        }
        try {
          op_canvas_2d_font_face_load(o.#raw, result.data, true);
        } catch (e) {
          FontFaceInternals.setError(o, e);
          return;
        }
        FontFaceInternals.setLoaded(o);
      });
    }
    return o.#innerLoaded.promise;
  }

  static inspect(inspect, options) {
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
};

export class FontFace extends Object {
  constructor(family, source, descriptors = undefined) {
    const prefix = "Failed to construct 'FontFace'";
    requiredArguments(arguments.length, 2, prefix);
    family = convertDOMString(family);
    source = convertDOMStringOrBufferSource(source);
    descriptors = convertFontFaceDescriptors(descriptors);
    const newTarget = capturePrototype(new.target, FontFace);
    const o = ObjectCreate(newTarget.prototype);
    new FontFaceInternals(
      o,
      source,
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
    );
    return o;
  }

  get family() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getFamily(this);
  }

  set family(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'family' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setFamily(this, value);
  }

  get style() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getStyle(this);
  }

  set style(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'style' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setStyle(this, value);
  }

  get weight() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getWeight(this);
  }

  set weight(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'weight' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setWeight(this, value);
  }

  get stretch() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getStretch(this);
  }

  set stretch(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'stretch' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setStretch(this, value);
  }

  get unicodeRange() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getUnicodeRange(this);
  }

  set unicodeRange(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'unicodeRange' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setUnicodeRange(this, value);
  }

  get featureSettings() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getFeatureSettings(this);
  }

  set featureSettings(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'featureSettings' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setFeatureSettings(this, value);
  }

  get variationSettings() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getVariationSettings(this);
  }

  set variationSettings(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'variationSettings' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setVariationSettings(this, value);
  }

  get display() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getDisplay(this);
  }

  set display(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'display' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setDisplay(this, value);
  }

  get ascentOverride() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getAscentOverride(this);
  }

  set ascentOverride(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'ascentOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setAscentOverride(this, value);
  }

  get descentOverride() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getDescentOverride(this);
  }

  set descentOverride(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'descentOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setDescentOverride(this, value);
  }

  get lineGapOverride() {
    FontFaceInternals.checkInstance(this);
    return FontFaceInternals.getLineGapOverride(this);
  }

  set lineGapOverride(value) {
    FontFaceInternals.checkInstance(this);
    const prefix = "Failed to set 'lineGapOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    FontFaceInternals.setLineGapOverride(this, value);
  }

  get status() {
    return FontFaceInternals.getStatus(this);
  }

  load() {
    try {
      FontFaceInternals.checkInstance(this);
      FontFaceInternals.load(this);
      return FontFaceInternals.getLoaded(this);
    } catch (e) {
      return PromiseReject(e);
    }
  }

  get loaded() {
    try {
      return FontFaceInternals.getLoaded(this);
    } catch (e) {
      return PromiseReject(e);
    }
  }

  get [privateCustomInspect]() {
    return FontFaceInternals.hasInstance(this)
      ? FontFaceInternals.inspect
      : undefined;
  }

  static {
    ObjectSetPrototypeOf(this, FunctionPrototype);
    configureInterface(this);
  }
}

const convertFontFace = (value) => {
  if (!(type(value) === "Object" && FontFaceInternals.hasInstance(value))) {
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

const FontFaceSetLoadEventInternals = class FontFaceSetLoadEvent
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
    const newTarget = capturePrototype(new.target, FontFaceSetLoadEvent);
    const o = ReflectConstruct(Event, [type, eventInitDict], newTarget);
    new FontFaceSetLoadEventInternals(o, ObjectFreeze(eventInitDict.fontfaces));
    return o;
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

const FontFaceSetInternals = class FontFaceSet extends IdentityConstructor {
  #brand() {}

  #raw;
  #setEntries = new SafeSet();
  #status = "loaded";
  #ready = new Deferred();
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

  static getSetEntries(o) {
    return o.#setEntries;
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
    if (o.#setEntries.has(font)) {
      return;
    }
    o.#setEntries.add(font);
    op_canvas_2d_font_face_set_insert(o.#raw, FontFaceInternals.getRaw(font));
    FontFaceInternals.addToSet(font, o);
    if (FontFaceInternals.getStatus(font) === "loading") {
      FontFaceSetInternals.addLoading(o, font);
    }
  }

  static delete(o, font) {
    if (!o.#setEntries.delete(font)) {
      return false;
    }
    op_canvas_2d_font_face_set_remove(o.#raw, FontFaceInternals.getRaw(font));
    o.#loadedFonts.delete(font);
    o.#failedFonts.delete(font);
    o.#loadingFonts.delete(font);
    if (o.#loadingFonts.size === 0) {
      FontFaceSetInternals.switchToLoaded(o);
    }
    return true;
  }

  static clear(o) {
    o.#setEntries.clear();
    op_canvas_2d_font_face_set_clear(o.#raw);
    o.#loadedFonts.clear();
    o.#failedFonts.clear();
    if (o.#loadingFonts.size !== 0) {
      o.#loadingFonts.clear();
      FontFaceSetInternals.switchToLoaded(o);
    }
    return true;
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
    if (o.#ready.state === "fulfilled") {
      o.#ready = new Deferred();
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

  static findMatchingFontFaces(o, font, text) {
    const matchIds = op_canvas_2d_font_face_set_match(o.#raw, font, text);
    const matches = [];
    // deno-lint-ignore prefer-primordials
    for (const font of o.#setEntries) {
      const id = op_canvas_2d_font_face_id(FontFaceInternals.getRaw(font));
      if (SetPrototypeHas(matchIds, id)) {
        ArrayPrototypePush(matches, font);
      }
    }
    return matches;
  }

  static inspect(inspect, options) {
    return inspect(
      new class FontFaceSet extends Set {
        onloading;
        onloadingdone;
        onloadingerror;
        ready;
        status;

        constructor(o) {
          super(o.#setEntries);
          this.onloading = o.#onloading.value;
          this.onloadingdone = o.#onloadingdone.value;
          this.onloadingerror = o.#onloadingerror.value;
          this.ready = o.#ready.promise;
          this.status = o.#status;
        }

        get [SymbolToStringTag]() {
          return "FontFaceSet";
        }
      }(this),
      options,
    );
  }
};

function makeSetlike(prototype, getSetEntries) {
  const setlike = ObjectGetOwnPropertyDescriptors({
    get size() {
      return SetPrototypeGetSize(getSetEntries(this));
    },
    entries() {
      return SetPrototypeEntries(getSetEntries(this));
    },
    values() {
      return SetPrototypeValues(getSetEntries(this));
    },
    forEach(callbackFn, thisArg = undefined) {
      return SetPrototypeForEach(getSetEntries(this), callbackFn, thisArg);
    },
    has(value) {
      return SetPrototypeHas(getSetEntries(this), value);
    },
  });
  ObjectDefineProperties(prototype, {
    size: setlike.size,
    [SymbolIterator]: setlike.values,
    entries: setlike.entries,
    keys: setlike.values,
    values: setlike.values,
    forEach: setlike.forEach,
    has: setlike.has,
  });
}

export class FontFaceSet extends EventTarget {
  // deno-lint-ignore constructor-super
  constructor() {
    illegalConstructor();
  }

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
    return FontFaceSetInternals.delete(this, font);
  }

  clear() {
    FontFaceSetInternals.checkInstance(this);
    FontFaceSetInternals.clear(this);
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
    try {
      FontFaceSetInternals.checkInstance(this);
      const prefix = "Failed to execute 'load' on 'FontFaceSet'";
      requiredArguments(arguments.length, 1, prefix);
      font = convertDOMString(font);
      text = convertDOMString(text);
      const fonts = FontFaceSetInternals
        .findMatchingFontFaces(this, font, text);
      const promises = [];
      for (const font of new SafeArrayIterator(fonts)) {
        ArrayPrototypePush(promises, FontFaceInternals.load(font));
      }
      return safePromiseAll(promises);
    } catch (e) {
      return PromiseReject(e);
    }
  }

  check(font, text = " ") {
    FontFaceSetInternals.checkInstance(this);
    const prefix = "Failed to execute 'check' on 'FontFaceSet'";
    requiredArguments(arguments.length, 1, prefix);
    font = convertDOMString(font);
    text = convertDOMString(text);
    const fonts = FontFaceSetInternals.findMatchingFontFaces(this, font, text);
    return ArrayPrototypeEvery(
      fonts,
      (font) => FontFaceInternals.getStatus(font) === "loaded",
    );
  }

  get ready() {
    try {
      return FontFaceSetInternals.getReady(this);
    } catch (e) {
      return PromiseReject(e);
    }
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
    makeSetlike(this.prototype, FontFaceSetInternals.getSetEntries);
    configureInterface(this);
  }
}

function createFontFaceSetFromRaw(raw) {
  const o = ReflectConstruct(EventTarget, [], FontFaceSet);
  new FontFaceSetInternals(o, raw);
  return o;
}

let fetchFont = (_url, cb) =>
  defer(() =>
    cb({
      success: false,
      reason: new DOMException("Unable to load font", "NetworkError"),
    })
  );

export function setFetchFont(fn) {
  fetchFont = fn;
}

let fonts;
export const getFonts = {
  "get fonts"() {
    if (this !== null && this !== undefined && this !== globalThis) {
      throw new TypeError("Illegal invocation");
    }
    fonts ??= createFontFaceSetFromRaw(op_canvas_2d_font_source());
    return fonts;
  },
}["get fonts"];
