use anyhow::Result;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub const FONT_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Display {
    pub buffer: [[bool; 64]; 32],
    pub canvas: Canvas<Window>,
    scale: u32,
}

impl Display {
    pub fn new(window: Window) -> Result<Self> {
        let mut canvas = window.into_canvas().present_vsync().build()?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        Ok(Self {
            buffer: [[false; 64]; 32],
            canvas,
            scale: 10,
        })
    }

    pub fn clear(&mut self) {
        self.buffer = [[false; 64]; 32];
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut turned_off = false;
        for (i, row) in sprite.iter().enumerate() {
            for j in 0..8 {
                let bit = (row >> (7 - j)) & 1;
                if bit == 1 {
                    let x = (x + j) % 64;
                    let y = (y + i) % 32;
                    if self.buffer[y][x] {
                        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                        turned_off = true;
                    } else {
                        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }
                    self.canvas
                        .fill_rect(Rect::new(
                            (x * self.scale as usize) as i32,
                            (y * self.scale as usize) as i32,
                            self.scale,
                            self.scale,
                        ))
                        .unwrap();
                    self.buffer[y][x] ^= true;
                }
            }
        }

        self.canvas.present();

        turned_off
    }
}
