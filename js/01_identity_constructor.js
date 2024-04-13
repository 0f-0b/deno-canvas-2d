import { primordials } from "ext:core/mod.js";

const { Object } = primordials;

export class IdentityConstructor extends Object {
  constructor(o) {
    return o;
  }
}
