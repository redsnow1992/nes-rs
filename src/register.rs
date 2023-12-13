use bitflags::bitflags;

#[derive(Debug)]
pub struct Accumulator(pub u8);

impl Accumulator {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_set(&self, n: u8) -> bool {
        let bit = 1 << n;
        self.0 & bit != 0
    }

    pub fn load(&mut self, v: u8) {
        self.0 = v;
    }
}

#[derive(Debug)]
pub struct RegisterX(pub u8);

pub struct Status(pub u8);

bitflags! {
    impl Status: u8 {
        const C = 0b0000_0001;
        const Z = 0b0000_0010;
        const I = 0b0000_0100;
        const D = 0b0000_1000;
        const B = 0b0001_0000;
        const V = 0b0010_0000;
        const N = 0b1000_0000;
    }
}

// impl Status {
//     pub fn set(&mut self, v: u8) {
//         self.0 = self.0 | v;
//     }

//     pub fn unset(&mut self)
// }
