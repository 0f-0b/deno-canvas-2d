import { primordials } from "ext:core/mod.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
export const IntlLocale = Intl.Locale;
export const IntlLocalePrototype = IntlLocale.prototype;
const proto = ObjectGetOwnPropertyDescriptors(IntlLocalePrototype);
export const IntlLocalePrototypeGetBaseName = uncurryThis(proto.baseName.get);
export const IntlLocalePrototypeGetScript = uncurryThis(proto.script.get);
export const IntlLocalePrototypeMaximize = uncurryThis(proto.maximize.value);
