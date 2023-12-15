use crate::{cpu::CPU, resettable::Resettable};

pub struct Memory(pub [u8; 0xFFFF]);

impl Memory {
    pub fn load(&mut self, offset: usize, program: &[u8]) {
        self.0[offset..(offset + program.len())].copy_from_slice(program);
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn read_u16(&self, pos: u16) -> u16 {
        let mut dst = [0u8; 2];
        dst.clone_from_slice(&self.0[(pos as usize)..=((pos + 1) as usize)]);
        u16::from_le_bytes(dst)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.0[addr as usize] = data;
    }

    pub fn write_u16(&mut self, pos: u16, data: u16) {
        self.0[(pos as usize)..=((pos + 1) as usize)].copy_from_slice(&data.to_le_bytes()[..]);
    }

    pub fn translate(&self, mode: &AddressingMode, cpu: &CPU) -> u16 {
        let pc = &cpu.pc;
        match mode {
            AddressingMode::Immediate => pc.into(),
            AddressingMode::ZeroPage => self.read(pc.into()) as u16,
            AddressingMode::ZeroPage_X => {
                let pos = self.read(pc.into());
                pos.wrapping_add(cpu.rx.0) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.read(pc.into());
                pos.wrapping_add(cpu.ry.0) as u16
            }
            AddressingMode::Absolute => self.read_u16(pc.into()),
            AddressingMode::Absolute_X => {
                let base = self.read_u16(pc.into());
                base.wrapping_add(cpu.rx.0 as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.read_u16(pc.into());
                base.wrapping_add(cpu.ry.0 as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.read(pc.into());

                let ptr: u8 = base.wrapping_add(cpu.rx.0);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.read(pc.into());

                let lo = self.read(base as u16);
                let hi = self.read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(cpu.ry.0 as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
}

impl Resettable for Memory {
    fn reset(&mut self) {
        self.0 = [0u8; 0xFFFF];
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn test_read_write() {
        let mut memory = Memory([0; 0xFFFF]);
        memory.write(3, 10);
        assert_eq!(memory.read(3), 10);
    }

    #[test]
    fn test_read_write_u16() {
        let mut memory = Memory([0; 0xFFFF]);
        memory.write_u16(3, 0x3412);
        assert_eq!(memory.read_u16(3), 0x3412);
        println!("{:?}", memory.read_u16(3));
    }
}
