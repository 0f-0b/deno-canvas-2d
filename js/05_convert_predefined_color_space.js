import { createEnumConverter } from "./04_create_enum_converter.js";

export const convertPredefinedColorSpace = createEnumConverter(
  "PredefinedColorSpace",
  ["srgb", "srgb-linear", "display-p3", "display-p3-linear"],
);
