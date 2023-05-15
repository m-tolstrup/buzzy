use arbitrary;

#[derive(arbitrary::Arbitrary, Debug)]
pub struct Instruction {
    op: u8,
    dst: u8,
    src: u8,
    off: i16,
    imm: i32,
}

impl Instruction {
    pub fn into_bytes(&self) -> Vec<u8> {
        let buffer = vec![
            self.op,
            self.src << 4 | self.dst,
            self.off                    as u8,
            (self.off >> 8)             as u8,
            self.imm                    as u8,
            (self.imm >> 8)             as u8,
            (self.imm >> 16)            as u8,
            (self.imm >> 25)            as u8,
        ];
        buffer
    }
}
