#![allow(dead_code)]

use std::time::Duration;

use anyhow::Result;
use chip8::Chip8;
use display::Display;

mod audio;
mod chip8;
mod display;
mod input;
mod opcode;

fn main() -> Result<()> {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();
    let window = video_subsystem
        .window("Chip8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let display = Display::new(window)?;

    let mut chip8 = Chip8::new(display);
    chip8.run("roms/programs/IBM Logo.ch8")?;

    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    Ok(())
}
