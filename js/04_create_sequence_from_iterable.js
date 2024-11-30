import { primordials } from "ext:core/mod.js";
import { requireObject } from "./01_require_object.js";

const {
  ArrayPrototype,
  ArrayPrototypePush,
  FunctionPrototypeCall,
  ObjectSetPrototypeOf,
} = primordials;

export function createSequenceFromIterable(iterable, method, convert) {
  const iterator = requireObject(FunctionPrototypeCall(method, iterable));
  const nextMethod = iterator.next;
  const sequence = ObjectSetPrototypeOf([], null);
  for (;;) {
    const next = requireObject(FunctionPrototypeCall(nextMethod, iterator));
    if (next.done) {
      break;
    }
    ArrayPrototypePush(sequence, convert(next.value));
  }
  return ObjectSetPrototypeOf(sequence, ArrayPrototype);
}
