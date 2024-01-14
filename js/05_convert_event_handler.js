export const convertEventHandler = (value) =>
  typeof value === "function" || typeof value === "object" ? value : null;
