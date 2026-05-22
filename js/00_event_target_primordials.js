import { core, primordials } from "ext:core/mod.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
const { loadExtScript } = core;
const { EventTarget } = loadExtScript("ext:deno_web/02_event.js");
export const EventTargetPrototype = EventTarget.prototype;
const proto = ObjectGetOwnPropertyDescriptors(EventTargetPrototype);
export const EventTargetPrototypeAddEventListener = uncurryThis(
  proto.addEventListener.value,
);
export const EventTargetPrototypeRemoveEventListener = uncurryThis(
  proto.removeEventListener.value,
);
