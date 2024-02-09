(async () => {
  const canvas = new OffscreenCanvas(100, 50);
  const ctx = canvas.getContext("2d");
  const gradient = ctx.createLinearGradient(0, 0, 100, 0);
  gradient.addColorStop(0, "#ff0");
  gradient.addColorStop(1, "#00f");
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, 100, 50);
  const png = await canvas.convertToBlob();
  return URL.createObjectURL(png);
})();
