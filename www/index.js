import * as wasm from "chip-9";
import { Cpu } from "chip-9";
import { memory } from "chip-9/chip_9_bg";

const cpu = Cpu.new();

const translateKeys = {
  49: 0x1, // 1
  50: 0x2, // 2
  51: 0x3, // 3
  52: 0xc, // 4
  81: 0x4, // Q
  87: 0x5, // W
  69: 0x6, // E
  82: 0xd, // R
  65: 0x7, // A
  83: 0x8, // S
  68: 0x9, // D
  70: 0xe, // F
  90: 0xa, // Z
  88: 0x0, // X
  67: 0xb, // C
  86: 0xf // V
};


const run = async () => {
  const WIDTH = 64;
  const HEIGHT = 32;
  const canvas = document.getElementById("chip-8");
  const ctx = canvas.getContext("2d");

  const programMemory = new Uint8Array(memory.buffer, cpu.memory_ptr(), 4096);
  const displayMemory = new Uint8Array(memory.buffer, cpu.display_ptr(), 4096);

  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, WIDTH, HEIGHT);

  const loadRom = (rom) =>
    fetch(`roms/${rom}`)
      .then((r) => r.arrayBuffer())
      .then((buffer) => {
        cpu.reset();
        const rom = new DataView(buffer, 0, buffer.byteLength);
        for (let i = 0; i < rom.byteLength; i++) {
          programMemory[0x200 + i] = rom.getUint8(i);
        }

        updateDisplay();
      });

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

  const runloop = () => {
    for (let i = 0; i < 10; i++) {
      cpu.process_opcode();
    }
    cpu.decrement_timers();
    updateDisplay();
    window.requestAnimationFrame(runloop);
  };

  loadRom("TETRIS");
  window.requestAnimationFrame(runloop);

  document.addEventListener("keydown", event => {
    cpu.key_down(translateKeys[event.keyCode]);
  });

  document.addEventListener("keyup", event => {
    cpu.key_up(translateKeys[event.keyCode]);
  });
};

run();