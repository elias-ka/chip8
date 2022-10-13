#[derive(Debug)]
pub struct OpCode {
    pub op: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

impl OpCode {
    pub fn new(b1: u8, b2: u8) -> Self {
        Self {
            op: (b1 >> 4) & 0xF,
            x: b1 & 0xF,
            y: (b2 >> 4) & 0xF,
            n: b2 & 0xF,
            nn: b2,
            nnn: ((b1 & 0xF) as u16) << 8 | b2 as u16,
        }
    }
}
