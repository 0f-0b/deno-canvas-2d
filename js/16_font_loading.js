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
  op_canvas_2d_font_face_set_new,
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
} from "ext:canvas_2d/00_ops.js";
import { IdentityConstructor } from "ext:canvas_2d/01_identity_constructor.js";
import {
  makeSpeciesSafePromise,
  newFromSpeciesSafePromise,
  safePromiseAll,
} from "ext:canvas_2d/01_promise.js";
import { requireObject } from "ext:canvas_2d/01_require_object.js";
import { isArrayBuffer } from "ext:canvas_2d/02_is_array_buffer.js";
import { isDataView } from "ext:canvas_2d/02_is_data_view.js";
import { isTypedArray } from "ext:canvas_2d/02_is_typed_array.js";
import { createDictionaryConverter } from "ext:canvas_2d/04_create_dictionary_converter.js";
import { createSequenceFromIterable } from "ext:canvas_2d/04_create_sequence_from_iterable.js";
import { convertArrayBuffer } from "ext:canvas_2d/05_convert_array_buffer.js";
import { convertDataView } from "ext:canvas_2d/05_convert_data_view.js";
import { convertDOMString } from "ext:canvas_2d/05_convert_dom_string.js";
import { convertEventHandler } from "ext:canvas_2d/05_convert_event_handler.js";
import { convertTypedArray } from "ext:canvas_2d/05_convert_typed_array.js";
import { EventHandler, readEventInitMembers } from "ext:canvas_2d/15_event.js";
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
  illegalConstructor,
  requiredArguments,
  type,
} from "ext:deno_webidl/00_webidl.js";

const {
  ArrayPrototypeEvery,
  ArrayPrototypePush,
  ObjectDefineProperties,
  ObjectGetOwnPropertyDescriptors,
  ObjectFreeze,
  PromiseReject,
  ReflectConstruct,
  SafeArrayIterator,
  SafeSet,
  Set,
  SetPrototypeGetSize,
  SetPrototypeEntries,
  SetPrototypeForEach,
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
let loadFontFace;

export class FontFace {
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

  constructor(family, source, descriptors = undefined) {
    family = convertDOMString(family);
    source = convertDOMStringOrBinaryData(source);
    descriptors = convertFontFaceDescriptors(descriptors);
    try {
      this.#url = typeof source === "string"
        ? op_canvas_2d_font_face_select_source(source)
        : null;
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
      );
    } catch (e) {
      this.#setError(e);
      this.#url = null;
      this.#raw = op_canvas_2d_font_face_errored();
      return;
    }
    if (typeof source === "string") {
      return;
    }
    defer(() => {
      this.#setLoading();
      defer(() => {
        try {
          op_canvas_2d_font_face_load(this.#raw, source, false);
        } catch (e) {
          this.#setError(e);
          return;
        }
        this.#setLoaded();
      });
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
    this.#cachedStyle ??= op_canvas_2d_font_face_style(this.#raw);
    return this.#cachedStyle;
  }

  set style(value) {
    this.#brand;
    const prefix = "Failed to set 'style' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_style(this.#raw, value);
    this.#cachedStyle = null;
  }

  get weight() {
    this.#brand;
    this.#cachedWeight ??= op_canvas_2d_font_face_weight(this.#raw);
    return this.#cachedWeight;
  }

  set weight(value) {
    this.#brand;
    const prefix = "Failed to set 'weight' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_weight(this.#raw, value);
    this.#cachedWeight = null;
  }

  get stretch() {
    this.#brand;
    this.#cachedStretch ??= op_canvas_2d_font_face_stretch(this.#raw);
    return this.#cachedStretch;
  }

  set stretch(value) {
    this.#brand;
    const prefix = "Failed to set 'stretch' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_stretch(this.#raw, value);
    this.#cachedStretch = null;
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
    this.#cachedFeatureSettings ??= op_canvas_2d_font_face_feature_settings(
      this.#raw,
    );
    return this.#cachedFeatureSettings;
  }

  set featureSettings(value) {
    this.#brand;
    const prefix = "Failed to set 'featureSettings' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_feature_settings(this.#raw, value);
    this.#cachedFeatureSettings = null;
  }

  get variationSettings() {
    this.#brand;
    this.#cachedVariationSettings = op_canvas_2d_font_face_variation_settings(
      this.#raw,
    );
    return this.#cachedVariationSettings;
  }

  set variationSettings(value) {
    this.#brand;
    const prefix = "Failed to set 'variationSettings' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_variation_settings(this.#raw, value);
    this.#cachedVariationSettings = null;
  }

  get display() {
    this.#brand;
    this.#cachedDisplay = op_canvas_2d_font_face_display(this.#raw);
    return this.#cachedDisplay;
  }

  set display(value) {
    this.#brand;
    const prefix = "Failed to set 'display' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_display(this.#raw, value);
    this.#cachedDisplay = null;
  }

  get ascentOverride() {
    this.#brand;
    this.#cachedAscentOverride = op_canvas_2d_font_face_ascent_override(
      this.#raw,
    );
    return this.#cachedAscentOverride;
  }

  set ascentOverride(value) {
    this.#brand;
    const prefix = "Failed to set 'ascentOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_ascent_override(this.#raw, value);
    this.#cachedAscentOverride = null;
  }

  get descentOverride() {
    this.#brand;
    this.#cachedDescentOverride = op_canvas_2d_font_face_descent_override(
      this.#raw,
    );
    return this.#cachedDescentOverride;
  }

  set descentOverride(value) {
    this.#brand;
    const prefix = "Failed to set 'descentOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_descent_override(this.#raw, value);
    this.#cachedDescentOverride = null;
  }

  get lineGapOverride() {
    this.#brand;
    this.#cachedLineGapOverride = op_canvas_2d_font_face_line_gap_override(
      this.#raw,
    );
    return this.#cachedLineGapOverride;
  }

  set lineGapOverride(value) {
    this.#brand;
    const prefix = "Failed to set 'lineGapOverride' on 'FontFace'";
    requiredArguments(arguments.length, 1, prefix);
    value = convertDOMString(value);
    op_canvas_2d_font_face_set_line_gap_override(this.#raw, value);
    this.#cachedLineGapOverride = null;
  }

  #setLoading() {
    this.#status = "loading";
    // deno-lint-ignore prefer-primordials
    for (const set of this.#fontFaceSets) {
      FontFaceSetInternals.addLoading(set, this);
    }
  }

  #setLoaded() {
    this.#innerLoaded.resolve(this);
    this.#status = "loaded";
    // deno-lint-ignore prefer-primordials
    for (const set of this.#fontFaceSets) {
      FontFaceSetInternals.addLoaded(set, this);
    }
    this.#fontFaceSets = null;
  }

  #setError(reason) {
    this.#innerLoaded.reject(reason);
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

  #load() {
    const url = this.#url;
    if (url !== null) {
      this.#url = null;
      this.#setLoading();
      fetchFont(url, (result) => {
        if (!result.success) {
          this.#setError(result.reason);
          return;
        }
        try {
          op_canvas_2d_font_face_load(this.#raw, result.data, true);
        } catch (e) {
          this.#setError(e);
          return;
        }
        this.#setLoaded();
      });
    }
    return this.#innerLoaded.promise;
  }

  load() {
    try {
      this.#load();
      return this.#loaded;
    } catch (e) {
      return PromiseReject(e);
    }
  }

  get loaded() {
    try {
      return this.#loaded;
    } catch (e) {
      return PromiseReject(e);
    }
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
    loadFontFace = (o) => o.#load();
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
    op_canvas_2d_font_face_set_insert(o.#raw, getFontFaceRaw(font));
    addFontFaceToSet(font, o);
    if (getFontFaceStatus(font) === "loading") {
      FontFaceSetInternals.addLoading(o, font);
    }
  }

  static delete(o, font) {
    if (!o.#setEntries.delete(font)) {
      return false;
    }
    op_canvas_2d_font_face_set_remove(o.#raw, getFontFaceRaw(font));
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
      const id = op_canvas_2d_font_face_id(getFontFaceRaw(font));
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
        ArrayPrototypePush(promises, loadFontFace(font));
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
      (font) => getFontFaceStatus(font) === "loaded",
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
export const getFonts = ({
  "get fonts"() {
    if (this !== null && this !== undefined && this !== globalThis) {
      throw new TypeError("Illegal invocation");
    }
    fonts ??= new FontFaceSet(illegalConstructor);
    return fonts;
  },
})["get fonts"];
