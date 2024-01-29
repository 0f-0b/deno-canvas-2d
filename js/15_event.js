import {
  EventPrototypeGetCurrentTarget,
  EventPrototypePreventDefault,
} from "ext:canvas_2d/00_event_primordials.js";
import {
  EventTargetPrototypeAddEventListener,
  EventTargetPrototypeRemoveEventListener,
} from "ext:canvas_2d/00_event_target_primordials.js";
import { convertBoolean } from "ext:canvas_2d/05_convert_boolean.js";
import { primordials } from "ext:core/mod.js";

const { FunctionPrototypeCall } = primordials;

export class EventHandler {
  target;
  name;
  value = null;
  listening = false;

  constructor(target, name) {
    this.target = target;
    this.name = name;
  }

  callback = (event) => {
    const callback = this.value;
    if (callback === null) {
      return;
    }
    const thisArg = EventPrototypeGetCurrentTarget(event);
    const returnValue = typeof callback === "function"
      ? FunctionPrototypeCall(callback, thisArg, event)
      : undefined;
    if (returnValue === false) {
      EventPrototypePreventDefault(event);
    }
  };

  update(value) {
    this.value = value;
    if (value === null) {
      if (!this.listening) {
        return;
      }
      EventTargetPrototypeRemoveEventListener(
        this.target,
        this.name,
        this.callback,
      );
      this.listening = false;
    } else {
      if (this.listening) {
        return;
      }
      EventTargetPrototypeAddEventListener(
        this.target,
        this.name,
        this.callback,
      );
      this.listening = true;
    }
  }
}

export const readEventInitMembers = (value) => {
  const result = { __proto__: null };
  const { bubbles = false } = value;
  result.bubbles = convertBoolean(bubbles);
  const { cancelable = false } = value;
  result.cancelable = convertBoolean(cancelable);
  const { composed = false } = value;
  result.composed = convertBoolean(composed);
  return result;
};
