extern crate core;

mod cpu;
mod memory;
mod cpu_aux;
mod disk;
mod keyboard;
mod mouse;

use cpu::CPU;
use memory::Memory;
use disk::Disk;
use keyboard::Keyboard;
use mouse::Mouse;

use crate::cpu_aux::TransferType;

use computer_config::Config;

pub struct Computer
{
    memory: Memory,

    // devices
    cpu: CPU,
    disk: Disk,
    keyboard: Keyboard,
    mouse: Mouse,
    //

    tt_bus: TransferType,
    addres_bus: u32,
    data_bus: u32,
}

impl Computer
{
    pub fn new(config: Config) -> Computer
    {
        Self::make_computer(
            config.rom_filename(),
            config.program_filename(),
            config.disk_filename(),
            config.disk_size(),
            config.memory_size(),
            config.vram_size(),
        )
    }
}

impl Computer
{
    fn make_computer
    (rom_filename: &Option<String>, program_filename: &Option<String>, disk_filename: &String, disk_size: u64,
     memory_size: u32, vram_size: u32)
     -> Computer
    {


        let cpu = CPU::new();
        let memory = Memory::new(rom_filename, program_filename, memory_size, vram_size);
        let disk = Disk::new(disk_size, &disk_filename);
        let keyboard = Keyboard::new();
        let mouse = Mouse::new();
        Computer
        {
            cpu,
            memory,
            disk,
            keyboard,
            mouse,
            tt_bus: TransferType::NoTransfer,
            addres_bus: 0,
            data_bus: 0
        }
    }



    pub fn cycle(&mut self)
    {
        self.cpu_tick(); // IF
        self.cpu_tick(); // DEXE
        self.cpu_tick(); // MEM
        self.cpu_tick(); // WB

        self.disk_controller();
        self.keyboard_controller();
        self.mouse_controller();
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

impl Computer // communication with devices (CPU, disk, display etc.)
{
    fn cpu_tick(&mut self)
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

    pub fn get_vram(&self) -> Vec<u8>
    {
        let mut vram: Vec<u8> = Vec::new();

        let from = self.memory.vram_start();
        let to = self.memory.vram_end();

        #[cfg(debug_assertions)]
        println!("from {from} to {to}");

        for address in from..to
        {
            let subpixel = self.memory.read_byte(address as usize);
            vram.push(subpixel);
        }

        #[cfg(debug_assertions)]
        println!("vram size: {}", vram.len());

        return vram;
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

        #[cfg(debug_assertions)]
        println!("disk controller: {} {} {}", tt, sec_num, data);

        match tt
        {
            0 => {}, // no transfer
            1 => {

                #[cfg(debug_assertions)]
                println!("Write {} to {}", data, sec_num);

                self.disk.write(sec_num, data);
                self.end_disk_transmission();
            },
            2 => {
                let data = self.disk.read(sec_num);
                let data_address = self.memory.disk_buffer_data_address();
                self.memory.write_word(data_address as usize, data); // write data to disk buffer
                self.end_disk_transmission();
            },
            _ => panic!("Bad disk transfer type"),
        }
    }

    fn end_disk_transmission(&mut self)
    {
        let tt_addr = self.memory.disk_buffer_transfer_type_address();
        self.memory.write_byte(tt_addr as usize, 0); // no transfer
    }

    fn keyboard_controller(&mut self)
    {
        let from = self.memory.keyboard_buffer_address();
        let to = self.memory.keyboard_buffer_end_address();
        for i in from..to
        {
            self.memory.write_byte(i as usize, 0);
        }

        let keys_pushed = self.keyboard.get_keys();
        for key in keys_pushed
        {
            let address = self.memory.keyboard_buffer_address() + key as u32;
            self.memory.write_byte(address as usize, 1);
        }
    }

    fn mouse_controller(&mut self)
    {
        let x_addr = self.memory.mouse_buffer_address();
        let y_addr = x_addr + 4;
        let lmb_addr = y_addr + 4;
        let rmb_addr = lmb_addr + 1;

        let (x, y, lmb, rmb) = self.mouse.get_mouse();

        self.memory.write_word(x_addr as usize, x);
        self.memory.write_word(y_addr as usize, y);
        self.memory.write_byte(lmb_addr as usize, lmb as u8);
        self.memory.write_byte(rmb_addr as usize, rmb as u8);
    }
}