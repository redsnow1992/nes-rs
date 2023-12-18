use crate::{
    register::{Accumulator, ProgramCounter, RegisterX, RegisterY, Status},
    resettable::Resettable,
};

pub struct CPU {
    pub ra: Accumulator,
    pub rx: RegisterX,
    pub ry: RegisterY,
    pub status: Status,
    pub pc: ProgramCounter,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl Resettable for CPU {
    /// restore the state of all registers, and initialize program_counter by the 2-byte value stored at 0xFFFC
    fn reset(&mut self) {
        self.ra.reset();
        self.rx.reset();
        self.status.reset();
        self.pc.reset();
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            ra: Accumulator(0),
            rx: RegisterX(0),
            ry: RegisterY(0),
            status: Status(0),
            pc: ProgramCounter(0),
        }
    }

    pub fn update_registers(&mut self) {
        self.status.set(Status::ZERO, self.ra.is_zero());
        self.status.set(Status::NEGATIVE, self.ra.is_set(7));
    }
}

#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_cpu_default() {
        let cpu = CPU::default();
        assert_eq!(cpu.ra.0, 0);
    }
}
