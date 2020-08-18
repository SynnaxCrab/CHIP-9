import * as wasm from "chip-9";

const canvas = document.getElementById("chip-8");
const ctx = canvas.getContext("2d");

ctx.fillStyle = "green";
ctx.fillRect(10, 10, 150, 100);

wasm.cpu();
