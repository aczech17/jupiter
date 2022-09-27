use std::fs::File;
use std::io::Read;
use crate::keyboard;
use keyboard::KEY_COUNT;

const DISK_BUFFER_SIZE: u32 = 1 + 8 + 4;
const MOUSE_BUFFER_SIZE: u32 = 4 + 4 + 1 + 1;

pub(crate) struct Memory
{
    data: Vec<u8>,
    size: u32,

    rom: (u32, u32),
    disk_buffer: (u32, u32),
    keyboard_buffer: (u32, u32),
    mouse_buffer: (u32, u32),
    vram: (u32, u32),
}

impl Memory
{
    pub fn new(rom_filename: &Option<String>, program_filename: &Option<String>, size: u32, vram_size: u32)
        -> Memory
    {
        let mut data: Vec<u8> = Vec::new();

        let rom_size = match rom_filename
        {
            None => 0,
            Some(filename) => {

                //eprintln!("Rom FILEname: {}", filename);
                let mut rom_file = File::open(filename).unwrap();
                rom_file.read_to_end(&mut data).unwrap();

                let file_size = rom_file.metadata().unwrap().len() as u32;
                file_size
            },
        };

        let (rom_start, rom_end) = (0, rom_size);

        let (disk_buffer_start, disk_buffer_end) =
            (rom_end as u32, rom_end as u32 + DISK_BUFFER_SIZE);
        let disk_buffer_size = disk_buffer_end - disk_buffer_start;
        for _ in 0..disk_buffer_size
        {
            data.push(0);
        }

        let (keyboard_buffer_start, keyboard_buffer_end) =
            (disk_buffer_end as u32, disk_buffer_end as u32 + KEY_COUNT as u32);
        let keyboard_buffer_size = keyboard_buffer_end - keyboard_buffer_start;
        for _ in 0..keyboard_buffer_size
        {
            data.push(0);
        }

        let (mouse_buffer_start, mouse_buffer_end) =
            (keyboard_buffer_end, keyboard_buffer_end + MOUSE_BUFFER_SIZE);
        let mouse_buffer_size = mouse_buffer_end - mouse_buffer_start;
        for _ in 0..mouse_buffer_size
        {
            data.push(0);
        }

        let (vram_start, vram_end) = (mouse_buffer_end, mouse_buffer_end + vram_size);
        let vram_size = vram_end - vram_start;
        for _ in 0..vram_size
        {
            data.push(0);
        }

        let program_size = match program_filename
        {
            None => 0,
            Some(filename) => {
                let mut program_file = File::open(filename).unwrap();
                program_file.read_to_end(&mut data).unwrap();
                let file_size = program_file.metadata().unwrap().len() as u32;
                file_size
            }
        };

        let program_end = vram_end + program_size;
        let size_left = size - program_end;
        for _ in 0..size_left
        {
            data.push(0);
        }

        Memory
        {
            data,
            size,
            rom: (rom_start, rom_end),
            disk_buffer: (disk_buffer_start, disk_buffer_end),
            keyboard_buffer: (keyboard_buffer_start, keyboard_buffer_end),
            mouse_buffer: (mouse_buffer_start, mouse_buffer_end),
            vram: (vram_start, vram_end),
        }
    }

    pub fn disk_buffer_transfer_type_address(&self) -> u32
    {
        return self.disk_buffer.0;
    }

    pub fn disk_buffer_data_address(&self) -> u32
    {
        return self.disk_buffer.0 + 9;
    }

    pub fn keyboard_buffer_address(&self) -> u32
    {
        return self.keyboard_buffer.0;
    }

    pub fn keyboard_buffer_end_address(&self) -> u32
    {
        return self.keyboard_buffer.1;
    }

    pub fn mouse_buffer_address(&self) -> u32
    {
        return self.mouse_buffer.0;
    }

    pub fn vram_start(&self) -> u32
    {
        return self.vram.0;
    }

    pub fn vram_end(&self) -> u32
    {
        return self.vram.1;
    }

    fn address_check(&self, address: usize)
    {
        if address as u32 >= self.size
        {
            panic!("Address exceeded memory size");
        }
    }
    
    pub fn read_byte(&self, address: usize) -> u8
    {
        self.address_check(address);
        self.data[address]
    }

    pub fn read_half(&self, address: usize) -> u16
    {
        self.address_check(address);
        ((self.data[address] as u16) << 8) | self.data[address] as u16
    }

    pub fn read_word(&self, address: usize) -> u32
    {
        self.address_check(address);
            ((self.data[address] as u32) << 24) |
            ((self.data[address + 1] as u32) << 16) |
            ((self.data[address + 2] as u32) << 8) |
            (self.data[address + 3] as u32)
    }

    fn write_address_check(&self, address: usize)
    {
        let address = address as u32;
        if address >= self.rom.0 && address <= self.rom.1
        {
            panic!("Memory read only");
        }
    }

    pub fn write_byte(&mut self, address: usize, data: u8)
    {
        #[cfg(debug_assertions)]
        println!("Writing {data} to {address}");

        self.address_check(address);
        self.write_address_check(address);
        self.data[address] = data;
    }

    pub fn write_half(&mut self, address: usize, data: u16)
    {
        self.address_check(address);
        self.write_address_check(address);
        self.data[address] = (data >> 8) as u8;
        self.data[address + 1] = (data & 0xFF) as u8;
    }

    pub fn write_word(&mut self, address: usize, data: u32)
    {
        self.address_check(address);
        self.write_address_check(address);
        self.data[address] = (data >> 24) as u8;
        self.data[address + 1] = ((data >> 16) & 0xFF) as u8;
        self.data[address + 2] = ((data >> 8) & 0xFF) as u8;
        self.data[address + 3] = (data & 0xFF) as u8;
    }
}
