pub struct ebpf_generator {
    pub number: i32,
}

impl ebpf_generator {
    pub fn new(num: i32) => ebpf_generator {
        ebpf_generator { number: num }
    }

    pub fn print(self) -> ! {
        println!("{:?}", number);
    }
}
