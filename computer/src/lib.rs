mod cpu;
mod memory;
mod cpu_aux;

use cpu::CPU;
use memory::Memory;
use crate::cpu_aux::TransferType;

pub struct Computer
{
    cpu: CPU,
    memory: Memory,
    tt_bus: TransferType,
    addres_bus: u32,
    data_bus: u32,
}

impl Computer
{
    pub fn new
    (rom_filename: Option<&str>, program_filename: Option<&str>, memory_size: u32, vram_size: u32)
     -> Computer
    {
        let cpu = CPU::new();
        let memory = Memory::new(rom_filename, program_filename, memory_size, vram_size);
        Computer
        {
            cpu,
            memory,
            tt_bus: TransferType::no_transfer,
            addres_bus: 0,
            data_bus: 0
        }
    }

    fn tick(&mut self)
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

        #[cfg(debug_assertions)]
        println!("{} {} {}", self.tt_bus as u8, self.addres_bus, self.data_bus);
    }

    pub fn cycle(&mut self)
    {
        self.tick(); // IF
        self.tick(); // DEXE
        self.tick(); // MEM
        self.tick(); // WB

        self.display();
    }

    fn display(&self)
    {
        let vram_start = self.memory.vram_start();
        let vram_end = self.memory.size();

        let pixel_count = (vram_end - vram_start) / 3;
        for i in 0..pixel_count
        {
            let (r, g, b) = self.memory.read_pixel(i);
            //println!("({}, {}, {})", r, g, b);
        }
    }

    pub fn run(mut self)
    {
        loop
        {
            self.cycle();
        }
    }
}