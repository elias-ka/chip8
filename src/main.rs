#![allow(dead_code)]

use std::env;

use anyhow::Result;
use chip8::Chip8;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window};

mod chip8;

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

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&args[1])?;

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { scancode, .. } => {
                    chip8.set_key(scancode.unwrap(), true);
                }
                Event::KeyUp { scancode, .. } => {
                    chip8.set_key(scancode.unwrap(), false);
                }
                _ => (),
            }
        }

        for _ in 0..10 {
            chip8.tick();
        }

        chip8.tick_timers();
        draw_screen(&chip8, &mut canvas)
    }

    Ok(())
}

fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in chip8.display.iter().enumerate() {
        if *pixel {
            canvas
                .fill_rect(Rect::new(
                    (i % 64) as i32 * 10,
                    (i / 64) as i32 * 10,
                    10,
                    10,
                ))
                .unwrap();
        }
    }

    canvas.present();
}
