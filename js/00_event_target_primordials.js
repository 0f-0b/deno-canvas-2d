import { primordials } from "ext:core/mod.js";
import { EventTarget } from "ext:deno_web/02_event.js";

const { ObjectGetOwnPropertyDescriptors, uncurryThis } = primordials;
export const EventTargetPrototype = EventTarget.prototype;
const proto = ObjectGetOwnPropertyDescriptors(EventTargetPrototype);
export const EventTargetPrototypeAddEventListener = uncurryThis(
  proto.addEventListener.value,
);
export const EventTargetPrototypeRemoveEventListener = uncurryThis(
  proto.removeEventListener.value,
);
