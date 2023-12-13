use crate::{
    code::Code,
    register::{Accumulator, RegisterX, Status},
};

pub struct CPU {
    pub ra: Accumulator,
    pub rx: RegisterX,
    pub status: Status,
    pc: u16,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            ra: Accumulator(0),
            rx: RegisterX(0),
            status: Status(0),
            pc: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;
        loop {
            let opscode = self.fetch(&program);
            let code: Code = num_traits::FromPrimitive::from_u8(opscode).unwrap();
            if code == Code::BRK {
                return;
            } else {
                code.execute(self, &program);
            }
        }
    }

    fn step(&mut self) {
        self.pc += 1;
    }

    pub fn fetch(&mut self, program: &[u8]) -> u8 {
        let code = program[self.pc as usize];
        self.step();
        code
    }
}

#[cfg(test)]
mod tests {
    use crate::register::{Accumulator, RegisterX};

    use super::CPU;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.ra.0, 0x05);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
        assert!(cpu.status.bits() & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.ra = Accumulator(10);
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.rx.0, 10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.rx.0, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.rx = RegisterX(0xff);
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.rx.0, 1)
    }
}
