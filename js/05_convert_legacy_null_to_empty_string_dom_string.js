export const convertLegacyNullToEmptyStringDOMString = (value) =>
  value === null ? "" : `${value}`;
