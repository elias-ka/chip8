#![allow(dead_code)]

use std::{env, time::Duration};

use anyhow::Result;
use chip8::Chip8;
use display::Display;
use sdl2::event::Event;

mod audio;
mod chip8;
mod display;
mod keypad;
mod opcode;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <rom>", args[0]);
        return Ok(());
    }

    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();
    let window = video_subsystem
        .window("CHIP-8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let display = Display::new(window)?;

    let mut chip8 = Chip8::new(display);
    chip8.run("roms/programs/IBM Logo.ch8")?;

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { scancode, .. } => {
                    chip8.keypad.set_key(scancode.unwrap(), true);
                }
                Event::KeyUp { scancode, .. } => {
                    chip8.keypad.set_key(scancode.unwrap(), false);
                }
                _ => {}
            }
        }

        for _ in 0..10 {
            chip8.execute_instruction()
        }

        chip8.tick_timers()
    }

    Ok(())
}
