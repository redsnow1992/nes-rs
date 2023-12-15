use crate::{
    memory::AddressingMode::{self, *},
    register::Status,
    system::System,
};

pub trait Oprand {
    fn execute(&self, system: &mut System);
    fn step_size(&self) -> u16;
}

pub struct LDA {
    pub mode: AddressingMode,
}

impl Oprand for LDA {
    fn execute(&self, system: &mut System) {
        let addr = system.mem.translate(&self.mode, &system.cpu);
        let param = system.fetch_from(addr);
        let cpu = &mut system.cpu;
        cpu.ra.load(param);
        cpu.status.set(Status::Z, cpu.ra.is_zero());
        cpu.status.set(Status::N, cpu.ra.is_set(7));
    }

    fn step_size(&self) -> u16 {
        match self.mode {
            Immediate => 1,
            ZeroPage => 1,
            Absolute => 2,
            _ => panic!("not set"),
        }
    }
}

pub struct TAX {
    pub mode: AddressingMode,
}

impl Oprand for TAX {
    fn execute(&self, system: &mut System) {
        let cpu = &mut system.cpu;
        cpu.rx.0 = cpu.ra.0;
        cpu.status.set(Status::Z, cpu.ra.is_zero());
        cpu.status.set(Status::N, cpu.ra.is_set(7));
    }

    fn step_size(&self) -> u16 {
        0
    }
}

pub struct INX {
    pub mode: AddressingMode,
}

impl Oprand for INX {
    fn execute(&self, system: &mut System) {
        let cpu = &mut system.cpu;
        cpu.rx.0 = cpu.rx.0.wrapping_add(1);
        cpu.status.set(Status::Z, cpu.ra.is_zero());
        cpu.status.set(Status::N, cpu.ra.is_set(7));
    }

    fn step_size(&self) -> u16 {
        0
    }
}
