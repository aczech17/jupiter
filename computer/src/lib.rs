mod cpu;
mod memory;
mod cpu_aux;
mod disk;

use cpu::CPU;
use memory::Memory;
use disk::Disk;
use crate::cpu_aux::TransferType;

pub struct Computer
{
    cpu: CPU,
    memory: Memory,
    disk: Disk,
    tt_bus: TransferType,
    addres_bus: u32,
    data_bus: u32,
}

impl Computer
{
    pub fn new
    (rom_filename: Option<String>, program_filename: Option<String>, disk_filename: String, disk_size: u64,
     memory_size: u32, vram_size: u32)
     -> Computer
    {
        let cpu = CPU::new();
        let memory = Memory::new(rom_filename, program_filename, memory_size, vram_size);
        let disk = Disk::new(disk_size, disk_filename);
        Computer
        {
            cpu,
            memory,
            disk,
            tt_bus: TransferType::NoTransfer,
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
            NoTransfer => {},

            ReadByte | ReadByteUnsigned =>
                self.data_bus = self.memory.read_byte(address) as u32,

            ReadHalf | ReadHalfUnsigned =>
                self.data_bus = self.memory.read_half(address) as u32,

            ReadWord =>
                self.data_bus = self.memory.read_word(address),

            WriteByte =>
                self.memory.write_byte(address, data as u8),

            WriteHalf =>
                self.memory.write_half(address, data as u16),

            WriteWord =>
                self.memory.write_word(address, data),
        }

        #[cfg(debug_assertions)]
        println!("Transfer Type: {}, Address: {} Data: {}", self.tt_bus as u8, self.addres_bus, self.data_bus);
    }

    pub fn cycle(&mut self)
    {
        self.tick(); // IF
        self.tick(); // DEXE
        self.tick(); // MEM
        self.tick(); // WB

        self.disk_controller();
    }


    pub fn get_disk_buffer(&self) -> (u8, u64, u32)
    {
        let start = self.memory.disk_buffer_transfer_type_address() as usize;

        let transfer_type = self.memory.read_byte(start);

        let sector_hi = self.memory.read_word(start + 1);
        let sector_lo = self.memory.read_word(start + 5);

        let sector = ((sector_hi as u64) << 32) | (sector_lo as u64);

        let data = self.memory.read_word(start + 9);

        return (transfer_type, sector, data);
    }

    fn disk_controller(&mut self)
    {
        let (tt, sec_num, data) = self.get_disk_buffer();
        match tt
        {
            0 => {}, // no transfer
            1 => self.disk.write(sec_num, data), // write to disk
            2 => {
                let data = self.disk.read(sec_num);
                let data_address = self.memory.disk_buffer_data_address();
                self.memory.write_word(data_address as usize, data); // write data to disk buffer
            },
            _ => panic!("Bad disk transfer type"),
        }

        // end of transmission
        let tt_addr = self.memory.disk_buffer_transfer_type_address();
        self.memory.write_byte(tt_addr as usize, 0); // no transfer
    }

    pub fn get_vram(&self) -> Vec<u8>
    {
        let mut data: Vec<u8> = Vec::new();

        let from = self.memory.vram_start();
        let to = self.memory.vram_end();

        for address in from..to
        {
            data.push(self.memory.read_byte(address as usize));
        }
        return data;
    }

    #[allow(unused)]
    pub fn run(mut self)
    {
        loop
        {
            self.cycle();
        }
    }
}