mod cpu;
mod memory;
mod cpu_aux;

use cpu::CPU;
use memory::Memory;
use crate::cpu_aux::TransferType;

struct Computer
{
    cpu: CPU,
    memory: Memory,
    tt_bus: TransferType,
    addres_bus: u32,
    data_bus: u32,
}

impl Computer
{
    pub fn new(memory_size: usize) -> Computer
    {
        let cpu = CPU::new();
        let memory = Memory::new(memory_size);
        Computer
        {
            cpu,
            memory,
            tt_bus: TransferType::no_transfer,
            addres_bus: 0,
            data_bus: 0
        }
    }

    fn tick(mut self)
    {
        (self.tt_bus, self.addres_bus, self.data_bus) = self.cpu.tick(self.data_bus); // read and receive

        let address = self.addres_bus as usize;
        let data = self.data_bus;

        use TransferType::*;
        match self.tt_bus
        {
            no_transfer => {},

            read_byte | read_byte_unsigned =>
                self.data_bus = self.memory.read_byte(address) as u32,

            read_half | read_half_unsigned =>
                self.data_bus = self.memory.read_half(address) as u32,

            read_word =>
                self.data_bus = self.memory.read_word(address),

            write_byte =>
                self.memory.write_byte(address, data as u8),

            write_half =>
                self.memory.write_half(address, data as u16),

            write_word =>
                self.memory.write_word(address, data),
        }
    }
}