mod cpu;
mod utils;

use cpu::Cpu;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn cpu() {
    let cpu = Cpu::new();
    let opcode = format!("Current OP code: {}", cpu.current_opcode());
    alert(&opcode);
}
