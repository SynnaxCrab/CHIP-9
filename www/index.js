import * as wasm from "chip-9";

const run = async () => {
  const WIDTH = 64;
  const HEIGHT = 32;
  const canvas = document.getElementById("chip-8");
  const ctx = canvas.getContext("2d");

  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, WIDTH, HEIGHT);
};

console.log(wasm.opcode());
alert(wasm.opcode());
run();
