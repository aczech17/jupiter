use std::fs::File;
use std::io::Read;


const DISK_BUFFER_SIZE: u32 = 1 + 8 + 4;
pub(crate) struct Memory
{
    data: Vec<u8>,
    size: u32,

    rom_size: u32,
    vram_size: u32,
}

impl Memory
{
    pub fn new(rom_filename: &Option<String>, program_filename: &Option<String>, size: u32, vram_size: u32)
        -> Memory
    {
        if vram_size % 3 != 0
        {
            panic!("VRAM not aligned by 3");
        }

        let mut data: Vec<u8> = Vec::new();

        let mut rom_size = 0;
        match rom_filename
        {
            Some(filename) => {
                let mut rom_file = File::open(filename).expect("Could not open rom file");
                rom_file.read_to_end(&mut data).expect("Could not read rom");

                rom_size = data.len();
                if rom_size as u32 > size
                {
                    panic!("Could not read rom. Memory size exceeded");
                }
            }

            None => {}, // No rom
        }
        // rom is ok

        let disk_buff_start = rom_size;
        let disk_buff_end = rom_size + DISK_BUFFER_SIZE as usize;
        let vram_end = disk_buff_end + vram_size as usize;

        // fill disk buffer and vram with zeros
        for _ in disk_buff_start..vram_end
        {
            data.push(0);
        }


        match program_filename
        {
            Some(filename) => {
                let mut program_file = File::open(filename).expect("Could not open program file");
                program_file.read_to_end(&mut data).expect("Could not read program");

                if data.len() as u32 > size
                {
                    panic!("Could not read program. Memory size exceeded");
                }

            }

            None => {}, // no program
        }
        let program_end = data.len();



        let size_left = size - program_end as u32;

        for _ in 0..size_left
        {
            data.push(0);
        }

        Memory
        {
            data,
            rom_size: rom_size as u32,
            vram_size,
            size,
        }
    }

    pub fn disk_buffer_transfer_type_address(&self) -> u32
    {
        return self.rom_size;
    }

    /*
    pub fn disk_buffer_sector_number_address(&self) -> u32
    {
        return self.rom_size + 1;
    }
     */

    pub fn disk_buffer_data_address(&self) -> u32
    {
        return self.rom_size + 9;
    }

    pub fn vram_start(&self) -> u32
    {
        return self.rom_size + DISK_BUFFER_SIZE;
    }

    pub fn vram_end(&self) -> u32
    {
        return self.vram_start() + self.vram_size;
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
        if (address as u32) < self.rom_size
        {
            panic!("Memory access violation kurwo! Cannot write to rom");
        }
    }

    pub fn write_byte(&mut self, address: usize, data: u8)
    {
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