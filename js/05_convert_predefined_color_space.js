import { createEnumConverter } from "./04_create_enum_converter.js";

export const convertPredefinedColorSpace = createEnumConverter(
  "PredefinedColorSpace",
  ["srgb", "display-p3"],
);
