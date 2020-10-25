import * as wasm from "chip-9";
import { memory } from "chip-9/chip_9_bg";

const run = async () => {
  const WIDTH = 64;
  const HEIGHT = 32;
  const canvas = document.getElementById("chip-8");
  const ctx = canvas.getContext("2d");

  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, WIDTH, HEIGHT);

  const imageData = ctx.createImageData(WIDTH, HEIGHT);
  ctx.putImageData(imageData, 0, 0);
};

console.log(wasm.opcode());
console.log(memory.buffer);
alert(wasm.opcode());
run();
