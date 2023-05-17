/* USED TO GATHER PERFORMANCE RESULTS WHEN GENERATING EBPF PROGRAMS CONSISTING OF COMPLETLY RANDOM BYTES */

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
            (self.imm >> 24)            as u8,
        ];
        buffer
    }
}

#[derive(arbitrary::Arbitrary, Debug)]
pub struct Map {
    map_type: i32,
    key_size: i32,
    value_size: i32,
    inner_idx: i32,
}

impl Map {
    pub fn into_bytes(&self) -> Vec<u8> {
        let buffer = vec![
            self.map_type                    as u8,
            (self.map_type >> 8)             as u8,
            (self.map_type >> 16)            as u8,
            (self.map_type >> 24)            as u8,
            self.key_size                    as u8,
            (self.key_size >> 8)             as u8,
            (self.key_size >> 16)            as u8,
            (self.key_size >> 24)            as u8,
            self.value_size                  as u8,
            (self.value_size >> 8)           as u8,
            (self.value_size >> 16)          as u8,
            (self.value_size >> 24)          as u8,
            self.inner_idx                   as u8,
            (self.inner_idx >> 8)            as u8,
            (self.inner_idx >> 16)           as u8,
            (self.inner_idx >> 24)           as u8,
        ];
        buffer
    }
}
