import { core, primordials } from "ext:core/mod.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
const { loadExtScript } = core;
const { Event } = loadExtScript("ext:deno_web/02_event.js");
export const EventPrototype = Event.prototype;
const proto = ObjectGetOwnPropertyDescriptors(EventPrototype);
export const EventPrototypeGetCurrentTarget = uncurryThis(
  proto.currentTarget.get,
);
export const EventPrototypePreventDefault = uncurryThis(
  proto.preventDefault.value,
);
