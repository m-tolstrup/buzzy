pub struct EbpfGenerator {
    pub number: i32,
}

impl EbpfGenerator {
    pub fn new(num: i32) -> EbpfGenerator {
        EbpfGenerator { number: num }
    }

    pub fn print(self) {
        println!("{:?}", self.number);
    }
}
