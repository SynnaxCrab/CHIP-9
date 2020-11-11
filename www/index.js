import * as wasm from "chip-9";
import { Cpu } from "chip-9";
import { memory } from "chip-9/chip_9_bg";

const cpu = Cpu.new();
cpu.set_pixel();

const run = async () => {
  const WIDTH = 64;
  const HEIGHT = 32;
  const canvas = document.getElementById("chip-8");
  const ctx = canvas.getContext("2d");

  const displayMemory = new Uint8Array(memory.buffer, cpu.display_ptr(), 4096);

  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, WIDTH, HEIGHT);

  const updateDisplay = () => {
    const imageData = ctx.createImageData(WIDTH, HEIGHT);
    for (let i = 0; i < displayMemory.length; i++) {
      imageData.data[i * 4] = displayMemory[i] === 1 ? 0x33 : 0;
      imageData.data[i * 4 + 1] = displayMemory[i] === 1 ? 0xff : 0;
      imageData.data[i * 4 + 2] = displayMemory[i] === 1 ? 0x66 : 0;
      imageData.data[i * 4 + 3] = 255;
    }
    ctx.putImageData(imageData, 0, 0);
  };

  updateDisplay();
};

//console.log(wasm.opcode());
//alert(wasm.opcode());
run();
