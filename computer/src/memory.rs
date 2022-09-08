struct Memory
{
    data: Vec<u8>,
}

impl Memory
{
    pub fn new(memory_size: usize) -> Memory
    {
        let mut data = Vec::new();
        for _ in 0..memory_size
        {
            data.push(0);
        }

        Memory { data }
    }

    pub fn read_byte(self, address: usize) -> u8
    {
        self.data[address]
    }

    pub fn read_half(self, address: usize) -> u16
    {
        ((self.data[address] as u16) << 8) | self.data[address] as u16
    }

    pub fn read_word(self, address: usize) -> u32
    {
            ((self.data[address] as u32) << 24) |
            ((self.data[address + 1] as u32) << 16) |
            ((self.data[address + 2] as u32) << 8) |
            (self.data[address + 3] as u32)
    }

    pub fn write_byte(mut self, address: usize, data: u8)
    {
        self.data[address] = data;
    }

    pub fn write_half(mut self, address: usize, data: u16)
    {
        self.data[address] = (data >> 8) as u8;
        self.data[address + 1] = (data & 0xFF) as u8;
    }

    pub fn write_word(mut self, address: usize, data: u32)
    {
        self.data[address] = (data >> 24) as u8;
        self.data[address + 1] = ((data >> 16) & 0xFF) as u8;
        self.data[address + 2] = ((data >> 8) & 0xFF) as u8;
        self.data[address + 3] = (data & 0xFF) as u8;
    }
}