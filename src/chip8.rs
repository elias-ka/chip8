use std::{fs::File, io::Read};

use anyhow::Result;
use sdl2::keyboard::Scancode;

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

const MEMORY_SIZE: usize = 4096;

pub struct Chip8 {
    pub display: [bool; 64 * 32],
    memory: [u8; MEMORY_SIZE],
    // program counter
    pc: usize,
    // index register
    i: usize,
    // variable registers
    v: [u8; 16],
    stack: Vec<u16>,
    keys: [bool; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0u8; MEMORY_SIZE];
        // load the font sprites into memory
        memory[0..FONT_SPRITES.len()].copy_from_slice(&FONT_SPRITES);

        Self {
            display: [false; 64 * 32],
            memory,
            // the CHIP-8 program starts at address 0x200, which is the first instruction
            pc: 0x200,
            i: 0,
            v: [0u8; 16],
            stack: vec![],
            keys: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute_instruction(opcode);
    }

    pub fn set_key(&mut self, key: Scancode, pressed: bool) {
        match key {
            Scancode::Num1 => self.keys[0x1] = pressed,
            Scancode::Num2 => self.keys[0x2] = pressed,
            Scancode::Num3 => self.keys[0x3] = pressed,
            Scancode::Num4 => self.keys[0xC] = pressed,
            Scancode::Q => self.keys[0x4] = pressed,
            Scancode::W => self.keys[0x5] = pressed,
            Scancode::E => self.keys[0x6] = pressed,
            Scancode::R => self.keys[0xD] = pressed,
            Scancode::A => self.keys[0x7] = pressed,
            Scancode::S => self.keys[0x8] = pressed,
            Scancode::D => self.keys[0x9] = pressed,
            Scancode::F => self.keys[0xE] = pressed,
            Scancode::Z => self.keys[0xA] = pressed,
            Scancode::X => self.keys[0x0] = pressed,
            Scancode::C => self.keys[0xB] = pressed,
            Scancode::V => self.keys[0xF] = pressed,
            _ => {}
        }
    }

    pub fn wait_for_key(&self) -> Option<u8> {
        for (i, key) in self.keys.iter().enumerate() {
            if *key {
                return Some(i as u8);
            }
        }

        None
    }

    pub fn load_rom(&mut self, rom_path: &str) -> Result<usize> {
        let mut rom = File::open(rom_path)?;
        Ok(rom.read(&mut self.memory[0x200..])?)
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc + 1] as u16;
        self.pc += 2;
        opcode
    }

    pub fn execute_instruction(&mut self, op: u16) {
        let nibble1 = (op & 0xF000) >> 12;
        let nibble2 = (op & 0x0F00) >> 8;
        let nibble3 = (op & 0x00F0) >> 4;
        let nibble4 = op & 0x000F;

        match (nibble1, nibble2, nibble3, nibble4) {
            // 00E0 - CLS
            (0, 0, 0xE, 0) => self.display = [false; 64 * 32],
            // 00EE - RET
            (0, 0, 0xE, 0xE) => {
                let addr = self.stack.pop().unwrap();
                self.pc = addr as usize;
            }
            // 1NNN - JP addr
            (1, _, _, _) => {
                let addr = op & 0x0FFF;
                self.pc = addr as usize;
            }
            // 2NNN - CALL addr
            (2, _, _, _) => {
                let addr = op & 0x0FFF;
                self.stack.push(self.pc as u16);
                self.pc = addr as usize;
            }
            // 3XNN - SE Vx, byte
            (3, x, _, _) => {
                let byte = (op & 0x00FF) as u8;
                if self.v[x as usize] == byte {
                    self.pc += 2;
                }
            }
            // 4XNN - SNE Vx, byte
            (4, x, _, _) => {
                let byte = (op & 0x00FF) as u8;
                if self.v[x as usize] != byte {
                    self.pc += 2;
                }
            }
            // 5XY0 - SE Vx, Vy
            (5, x, y, 0) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            // 6XNN - LD Vx, byte
            (6, x, _, _) => {
                let byte = (op & 0x00FF) as u8;
                self.v[x as usize] = byte;
            }
            // 7XNN - ADD Vx, byte
            (7, x, _, _) => {
                let byte = (op & 0x00FF) as u8;
                self.v[x as usize] = self.v[x as usize].wrapping_add(byte);
            }
            // 8XY0 - LD Vx, Vy
            (8, x, y, 0) => self.v[x as usize] = self.v[y as usize],
            // 8XY1 - OR Vx, Vy
            (8, x, y, 1) => self.v[x as usize] |= self.v[y as usize],
            // 8XY2 - AND Vx, Vy
            (8, x, y, 2) => self.v[x as usize] &= self.v[y as usize],
            // 8XY3 - XOR Vx, Vy
            (8, x, y, 3) => self.v[x as usize] ^= self.v[y as usize],
            // 8XY4 - ADD Vx, Vy
            (8, x, y, 4) => {
                let (result, overflow) = self.v[x as usize].overflowing_add(self.v[y as usize]);
                self.v[x as usize] = result;
                self.v[0xF] = if overflow { 1 } else { 0 };
            }
            // 8XY5 - SUB Vx, Vy
            (8, x, y, 5) => {
                let (result, overflow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                self.v[x as usize] = result;
                self.v[0xF] = if overflow { 0 } else { 1 };
            }
            // 8XY6 - SHR Vx {, Vy}
            (8, x, _, 6) => {
                self.v[0xF] = self.v[x as usize] & 0x1;
                self.v[x as usize] >>= 1;
            }
            // 8XY7 - SUBN Vx, Vy
            (8, x, y, 7) => {
                let (result, overflow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                self.v[x as usize] = result;
                self.v[0xF] = if overflow { 0 } else { 1 };
            }
            // 8XYE - SHL Vx {, Vy}
            (8, x, _, 0xE) => {
                self.v[0xF] = self.v[x as usize] >> 7;
                self.v[x as usize] <<= 1;
            }
            // 9XY0 - SNE Vx, Vy
            (9, x, y, 0) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            // ANNN - LD I, addr
            (0xA, _, _, _) => {
                let addr = op & 0x0FFF;
                self.i = addr as usize;
            }
            // BNNN - JP V0, addr
            (0xB, _, _, _) => {
                let addr = op & 0x0FFF;
                self.pc = (addr + self.v[0] as u16) as usize;
            }
            // CXNN - RND Vx, byte
            (0xC, x, _, _) => {
                let byte = (op & 0x00FF) as u8;
                let random = rand::random::<u8>();
                self.v[x as usize] = random & byte;
            }
            // DXYN - DRW Vx, Vy, nibble
            (0xD, x, y, n) => {
                let x = self.v[x as usize] as usize;
                let y = self.v[y as usize] as usize;

                self.v[0xF] = 0;

                for dy in 0..n as usize {
                    let sprite = self.memory[self.i + dy];
                    for dx in 0..8 {
                        let pixel = (sprite >> (7 - dx)) & 0x1;
                        if pixel != 0 {
                            let x = (x + dx) % 64;
                            let y = (y + dy) % 32;
                            let index = x + y * 64;
                            if self.display[index] {
                                self.v[0xF] = 1;
                            }
                            self.display[index] = !self.display[index];
                        }
                    }
                }
            }
            // EX9E - SKP Vx
            (0xE, x, 0x9, 0xE) => {
                if self.keys[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
            }
            // EXA1 - SKNP Vx
            (0xE, x, 0xA, 0x1) => {
                if !self.keys[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
            }
            // FX07 - LD Vx, DT
            (0xF, x, 0x0, 0x7) => self.v[x as usize] = self.delay_timer,
            // FX0A - LD Vx, K
            (0xF, x, 0x0, 0xA) => {
                for (i, key) in self.keys.iter().enumerate() {
                    if *key {
                        self.v[x as usize] = i as u8;
                        return;
                    }
                }
                self.pc -= 2;
            }
            // FX15 - LD DT, Vx
            (0xF, x, 0x1, 0x5) => self.delay_timer = self.v[x as usize],
            // FX18 - LD ST, Vx
            (0xF, x, 0x1, 0x8) => self.delay_timer = self.v[x as usize],
            // FX1E - ADD I, Vx
            (0xF, x, 0x1, 0xE) => self.i += self.v[x as usize] as usize,
            // FX29 - LD F, Vx
            (0xF, x, 0x2, 0x9) => self.i = self.v[x as usize] as usize * 5,
            // FX33 - LD B, Vx
            (0xF, x, 0x3, 0x3) => {
                let vx = self.v[x as usize];
                self.memory[self.i] = vx / 100;
                self.memory[self.i + 1] = (vx / 10) % 10;
                self.memory[self.i + 2] = vx % 10;
            }
            // FX55 - LD [I], Vx
            (0xF, x, 0x5, 0x5) => {
                for i in 0..=x {
                    self.memory[self.i + i as usize] = self.v[i as usize];
                }
            }
            // FX65 - LD Vx, [I]
            (0xF, x, 0x6, 0x5) => {
                for i in 0..=x {
                    self.v[i as usize] = self.memory[self.i + i as usize];
                }
            }
            _ => panic!("Unknown opcode: {:X}", op),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO: play sound
            }
            self.sound_timer -= 1
        }
    }
}
