use bitflags::bitflags;

use crate::resettable::Resettable;

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

impl Resettable for Accumulator {
    fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Debug)]
pub struct RegisterX(pub u8);

impl Resettable for RegisterX {
    fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Debug)]
pub struct RegisterY(pub u8);

impl Resettable for RegisterY {
    fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Debug)]
pub struct Status(pub u8);

impl Resettable for Status {
    fn reset(&mut self) {
        self.0 = 0;
    }
}

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

#[derive(Debug)]
pub struct ProgramCounter(pub u16);

impl ProgramCounter {
    pub fn step(&mut self) {
        self.step_n(1);
    }

    pub fn step_n(&mut self, n: u16) {
        self.0 += n;
    }
}

impl Resettable for ProgramCounter {
    fn reset(&mut self) {
        self.0 = 0xFFFC;
    }
}

impl From<&ProgramCounter> for u16 {
    fn from(pc: &ProgramCounter) -> Self {
        pc.0
    }
}
