use crate::{
    cpu::CPU,
    memory::{AddressingMode::*, Memory},
    oprand::*,
    resettable::Resettable,
};

pub struct System {
    pub cpu: CPU,
    pub mem: Memory,
}

impl System {
    pub fn pc(&self) -> u16 {
        let pc = &self.cpu.pc;
        pc.into()
    }
}

impl Resettable for System {
    fn reset(&mut self) {
        self.cpu.reset();
        self.cpu.pc.0 = self.mem.read_u16(0xFFFC);
    }
}

impl Default for System {
    fn default() -> Self {
        System::new()
    }
}

impl System {
    pub fn new() -> Self {
        Self {
            cpu: CPU::default(),
            mem: Memory([0u8; 0xFFFF]),
        }
    }

    /// load a program into PRG ROM space and save the reference to the code into 0xFFFC memory cell
    pub fn load(&mut self, program: Vec<u8>) {
        self.mem.load(0x8000, &program);
        self.mem.write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.fetch();
            let program_counter_state = self.pc();
            if opscode == 0x00 {
                return;
            } else {
                let oprand = Self::dispatch(opscode);
                oprand.execute(self);
                if program_counter_state == self.pc() {
                    self.cpu.pc.step_n(oprand.step_size());
                }
            }
        }
    }

    fn dispatch(code: u8) -> Box<dyn Oprand> {
        match code {
            0xA9 => Box::new(LDA { mode: Immediate }),
            0xA5 => Box::new(LDA { mode: ZeroPage }),
            0xAD => Box::new(LDA { mode: Absolute }),
            0xAA => Box::new(TAX { mode: Immediate }),
            0xE8 => Box::new(INX { mode: Immediate }),
            _ => panic!("not implement"),
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let code = self.mem.read(self.pc());
        self.cpu.pc.step();
        code
    }

    pub fn fetch_from(&mut self, addr: u16) -> u8 {
        let code = self.mem.read(addr);
        self.cpu.pc.step();
        code
    }
}

#[cfg(test)]
mod tests {
    use super::System;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut system = System::new();
        system.load_and_run(vec![0xa9, 0x05, 0x00]);
        let cpu = system.cpu;
        assert_eq!(cpu.ra.0, 0x05);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
        assert!(cpu.status.bits() & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut system = System::new();
        system.load_and_run(vec![0xa9, 0x00, 0x00]);

        let cpu = &system.cpu;
        assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut system = System::new();
        system.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        let cpu = &system.cpu;
        assert_eq!(cpu.rx.0, 10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut system = System::new();
        system.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        let cpu = &system.cpu;
        assert_eq!(cpu.rx.0, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut system = System::new();
        system.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        let cpu = &system.cpu;
        assert_eq!(cpu.rx.0, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut system = System::new();
        system.mem.write(0x10, 0x55);
        system.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(system.cpu.ra.0, 0x55);
    }
}
