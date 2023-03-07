pub struct ElfParser {
    pub number: i32,
}

impl ElfParser {
    pub fn new(num: i32) -> ElfParser {
        ElfParser { number: num }
    }

    pub fn print(self) {
        println!("{:?}", self.number);
    }
}