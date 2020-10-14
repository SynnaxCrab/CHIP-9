use rand::prelude::*;

fn main() {
    let mut memory: [u8; 4096] = [0; 4096];

    memory[0] = 0xA2;
    memory[1] = 0xF0;
    let k1 = memory[0] as u16;
    let k2 = memory[1] as u16;
    let kk = k1 << 8 | k2;
    //println!("Hello, world! {}::{}::{}", memory[0], memory[1], kk);
    println!(
        "Hello, world! {:#X}::{:#X}::{:#X}",
        memory[0], memory[1], kk
    );

    let a1: u8 = 11;
    let a2: u8 = 12;

    let (aa, overflow) = a1.overflowing_sub(a2);

    println!("{}::{}", aa, overflow);

    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1, 7);
    println!("{}", number);

    println!("{}", false as u8);
}
