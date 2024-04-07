import { primordials } from "ext:core/mod.js";
import { requireObject } from "./01_require_object.js";

const { ArrayPrototypePush, FunctionPrototypeCall } = primordials;

export function createSequenceFromIterable(iterable, method, convert) {
  const iterator = requireObject(FunctionPrototypeCall(method, iterable));
  const nextMethod = iterator.next;
  const sequence = [];
  for (;;) {
    const next = requireObject(FunctionPrototypeCall(nextMethod, iterator));
    if (next.done) {
      break;
    }
    ArrayPrototypePush(sequence, convert(next.value));
  }
  return sequence;
}
