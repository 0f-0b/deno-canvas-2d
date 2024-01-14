import { primordials } from "ext:core/mod.js";
import { Event } from "ext:deno_web/02_event.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
export const EventPrototype = Event.prototype;
const proto = ObjectGetOwnPropertyDescriptors(EventPrototype);
export const EventPrototypeGetCurrentTarget = uncurryThis(
  proto.currentTarget.get,
);
export const EventPrototypePreventDefault = uncurryThis(
  proto.preventDefault.value,
);
