use std::fs::File;
use std::os::windows::fs::FileExt;

pub(crate) struct Disk
{
    disk_file: File,
    size: u64,
}

impl Disk
{
    pub(crate) fn new(size: u64, filename: &str) -> Disk
    {
        if size % 4 != 0
        {
            panic!("Size should be divisible by 4");
        }
        match File::open(filename)
        {
            Ok(file) => return Disk {disk_file: file, size},
            Err(_) => {}, // go on and create a new file
        }
        let file = File::create(filename).expect("Could not create disk file");
        Disk
        {
            disk_file: file,
            size,
        }
    }

    pub(crate) fn read(&self, sector_num: u64) -> u32
    {
        let max_sector_number = self.size / 4;
        if sector_num > max_sector_number
        {
            panic!("Bad sector read");
        }
        let mut buf: [u8; 4] = [0; 4];

        self.disk_file.seek_read(&mut buf, sector_num).expect("DUPA");

        let sector = ((buf[0] as u32) << 24) |
            ((buf[1] as u32) << 16) |
            ((buf[2] as u32) << 8) |
            (buf[3] as u32);

        return sector;
    }

    pub(crate) fn write(&mut self, sector_num: u64, data: u32)
    {
        let max_sector_number = self.size / 4;
        if sector_num > max_sector_number
        {
            panic!("Bad sector write");
        }

        let buf: [u8; 4] = [(data >> 24) as u8, ((data >> 16) & 0xFF) as u8,
            ((data >> 8) & 0xFF) as u8, (data & 0xFF) as u8];

        self.disk_file.seek_write(&buf, sector_num).expect("PIZDA");
    }
}