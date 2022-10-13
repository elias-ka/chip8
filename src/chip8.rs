use std::{fs::File, io::Read};

use anyhow::Result;

use crate::{
    display::{Display, FONT_SPRITES},
    opcode::OpCode,
};

const MEMORY_SIZE: usize = 4096;

pub struct Chip8 {
    pub display: Display,
    memory: [u8; MEMORY_SIZE],
    // program counter
    pc: usize,
    // index register
    ir: usize,
    // variable registers
    vrs: [u8; 16],
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
            memory,
            // the CHIP-8 program starts at address 0x200, which is the first instruction
            pc: 0x200,
            ir: 0,
            vrs: [0u8; 16],
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn run(&mut self, rom_path: &str) -> Result<()> {
        self.load_rom(rom_path)?;
        loop {
            let opcode = self.fetch_opcode();
            self.decode_opcode(opcode);
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

    fn decode_opcode(&mut self, opcode: OpCode) {
        dbg!("{:?}", &opcode);
        match opcode.op {
            0x0 => match opcode.nn {
                0xE0 => self.display.clear(),
                _ => panic!("Unknown opcode (NN): {:#X}", opcode.nn),
            },
            0x1 => self.pc = opcode.nnn as usize,
            0x6 => self.vrs[opcode.x as usize] = opcode.nn,
            0x7 => self.vrs[opcode.x as usize] += opcode.nn,
            0xA => self.ir = opcode.nnn as usize,
            0xD => {
                let x = self.vrs[opcode.x as usize] as usize;
                let y = self.vrs[opcode.y as usize] as usize;
                let sprite = &self.memory[self.ir..self.ir + opcode.n as usize];
                self.vrs[0xF] = self.display.draw(x, y, sprite) as u8;
            }
            _ => panic!("Unknown opcode (OP): {:#X}", opcode.op),
        }
    }
}
