use crate::{cpu::CPU, register::Status};

#[derive(FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Code {
    TAX = 0xAA,
    LDA = 0xA9,
    BRK = 0x00,
    INX = 0xe8,
}

impl Code {
    pub fn execute(&self, cpu: &mut CPU, program: &[u8]) {
        match self {
            Code::TAX => handle_tax(cpu),
            Code::LDA => handle_lda(cpu, program),
            Code::INX => handle_inx(cpu),
            Code::BRK => (),
        }
    }
}

fn handle_lda(cpu: &mut CPU, program: &[u8]) {
    let param = cpu.fetch(program);
    cpu.ra.load(param);
    cpu.status.set(Status::Z, cpu.ra.is_zero());
    cpu.status.set(Status::N, cpu.ra.is_set(7));
}

fn handle_tax(cpu: &mut CPU) {
    cpu.rx.0 = cpu.ra.0;
    cpu.status.set(Status::Z, cpu.ra.is_zero());
    cpu.status.set(Status::N, cpu.ra.is_set(7));
}

fn handle_inx(cpu: &mut CPU) {
    cpu.rx.0 = cpu.rx.0.wrapping_add(1);
    cpu.status.set(Status::Z, cpu.ra.is_zero());
    cpu.status.set(Status::N, cpu.ra.is_set(7));
}
