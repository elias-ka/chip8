use std::{fs::File, io::Read};

use anyhow::Result;
use rand::random;

use crate::{
    display::{Display, FONT_SPRITES},
    keypad::Keypad,
    opcode::OpCode,
};

const MEMORY_SIZE: usize = 4096;

pub struct Chip8 {
    pub display: Display,
    pub keypad: Keypad,
    memory: [u8; MEMORY_SIZE],
    // program counter
    pc: usize,
    // index register
    i: usize,
    // variable registers
    v: [u8; 16],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new(display: Display) -> Self {
        let mut memory = [0u8; MEMORY_SIZE];
        // load the font sprites into memory
        memory[0..FONT_SPRITES.len()].copy_from_slice(&FONT_SPRITES);

        Self {
            display,
            keypad: Keypad::new(),
            memory,
            // the CHIP-8 program starts at address 0x200, which is the first instruction
            pc: 0x200,
            i: 0,
            v: [0u8; 16],
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn run(&mut self, rom_path: &str) -> Result<()> {
        self.load_rom(rom_path)?;
        loop {
            self.execute_instruction();
        }
    }

    fn load_rom(&mut self, rom_path: &str) -> Result<usize> {
        let mut rom = File::open(rom_path)?;
        Ok(rom.read(&mut self.memory[0x200..])?)
    }

    fn fetch_opcode(&mut self) -> OpCode {
        let opcode = OpCode::new(self.memory[self.pc], self.memory[self.pc + 1]);
        self.pc += 2;
        opcode
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.fetch_opcode();

        match opcode.op {
            0x0 => match opcode.nn {
                0xE0 => self.display.clear(),
                0xEE => self.pc = self.stack.pop().unwrap() as usize,
                _ => panic!("Unknown opcode (NN): {:#X}", opcode.nn),
            },
            0x1 => self.pc = opcode.nnn as usize,
            0x2 => {
                self.stack.push(self.pc as u16);
                self.pc = opcode.nnn as usize;
            }
            0x3 => {
                if self.v[opcode.x as usize] == opcode.nn {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.v[opcode.x as usize] != opcode.nn {
                    self.pc += 2;
                }
            }
            0x5 => {
                if self.v[opcode.x as usize] == self.v[opcode.y as usize] {
                    self.pc += 2;
                }
            }
            0x6 => self.v[opcode.x as usize] = opcode.nn,
            0x7 => self.v[opcode.x as usize] += opcode.nn,
            0x8 => match opcode.n {
                0x0 => self.v[opcode.x as usize] = self.v[opcode.y as usize],
                0x1 => self.v[opcode.x as usize] |= self.v[opcode.y as usize],
                0x2 => self.v[opcode.x as usize] &= self.v[opcode.y as usize],
                0x3 => self.v[opcode.x as usize] ^= self.v[opcode.y as usize],
                0x4 => {
                    let (result, overflow) =
                        self.v[opcode.x as usize].overflowing_add(self.v[opcode.y as usize]);
                    self.v[opcode.x as usize] = result;
                    self.v[0xF] = overflow as u8;
                }
                0x5 => {
                    let (result, overflow) =
                        self.v[opcode.x as usize].overflowing_sub(self.v[opcode.y as usize]);
                    self.v[opcode.x as usize] = result;
                    self.v[0xF] = !overflow as u8;
                }
                0x6 => {
                    self.v[0xF] = self.v[opcode.x as usize] & 0x1;
                    self.v[opcode.x as usize] >>= 1;
                }
                0x7 => {
                    let (result, overflow) =
                        self.v[opcode.y as usize].overflowing_sub(self.v[opcode.x as usize]);
                    self.v[opcode.x as usize] = result;
                    self.v[0xF] = !overflow as u8;
                }
                0xE => {
                    self.v[0xF] = self.v[opcode.x as usize] >> 7;
                    self.v[opcode.x as usize] <<= 1;
                }
                _ => panic!("Unknown opcode (N): {:#X}", opcode.n),
            },
            0xA => self.i = opcode.nnn as usize,
            0xB => self.pc = (opcode.nnn + self.v[0] as u16) as usize,
            0xC => self.v[opcode.x as usize] = random::<u8>() & opcode.nn,
            0xD => {
                let x = self.v[opcode.x as usize] as usize;
                let y = self.v[opcode.y as usize] as usize;
                let sprite = &self.memory[self.i..self.i + opcode.n as usize];
                self.v[0xF] = self.display.draw(x, y, sprite) as u8;
            }
            0xE => match opcode.nn {
                0x9E => {
                    if self.keypad.keys[self.v[opcode.x as usize] as usize] {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    if !self.keypad.keys[self.v[opcode.x as usize] as usize] {
                        self.pc += 2;
                    }
                }
                0x07 => self.v[opcode.x as usize] = self.delay_timer,
                0x15 => self.delay_timer = self.v[opcode.x as usize],
                0x18 => self.sound_timer = self.v[opcode.x as usize],
                _ => panic!("Unknown opcode (NN): {:#X}", opcode.nn),
            },
            0xF => match opcode.nn {
                0x07 => self.v[opcode.x as usize] = self.delay_timer,
                0x15 => self.delay_timer = self.v[opcode.x as usize],
                0x18 => self.sound_timer = self.v[opcode.x as usize],
                0x29 => self.i = self.v[opcode.x as usize] as usize * 5,
                0x33 => {
                    self.memory[self.i] = self.v[opcode.x as usize] / 100;
                    self.memory[self.i + 1] = (self.v[opcode.x as usize] / 10) % 10;
                    self.memory[self.i + 2] = (self.v[opcode.x as usize] % 100) % 10;
                }
                0x55 => {
                    for j in 0..=opcode.x {
                        self.memory[self.i + j as usize] = self.v[j as usize];
                    }
                }
                0x65 => {
                    for j in 0..=opcode.x {
                        self.v[j as usize] = self.memory[self.i + j as usize];
                    }
                }
                0x0A => {
                    if let Some(key) = self.keypad.wait_for_key() {
                        self.v[opcode.x as usize] = key;
                    } else {
                        self.pc -= 2;
                    }
                }
                0x1E => self.i += self.v[opcode.x as usize] as usize,
                _ => panic!("Unknown opcode (NN): {:#X}", opcode.nn),
            },
            _ => panic!("Unknown opcode (OP): {:#X}", opcode.op),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // play sound
            }
            self.sound_timer -= 1
        }
    }
}
