use std::fs::File;
use std::fs::OpenOptions;

#[cfg(target_os = "linux")]
use std::os::unix::fs::FileExt;

#[cfg(target_os = "windows")]
use std::os::windows::fs::FileExt;

pub(crate) struct Disk
{
    filename: String,
    size: u64,
}

impl Disk
{
    pub(crate) fn new(size: u64, filename: &String) -> Disk
    {
        if size % 4 != 0
        {
            panic!("Size should be divisible by 4");
        }
        match File::open(filename.clone())
        {
            Ok(_) => {}, // file already exists
            Err(_) => {
                File::create(filename.clone()).expect("Could not create a disk");
            },
        }

        Disk
        {
            filename: filename.clone(),
            size
        }
    }

    pub(crate) fn read(&self, sector_num: u64) -> u32
    {
        let max_sector_number = self.size / 4;
        if sector_num > max_sector_number
        {
            panic!("Bad sector read");
        }

        let file = File::open(&self.filename).expect("Could not open the disk");
        let mut buf: [u8; 4] = [0; 4];
        file.seek_read(&mut buf, sector_num).expect("Could not read from the disk");

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

        let file = OpenOptions::new()
            .append(false)
            .write(true)
            .open(&self.filename)
            .expect("Could not open the file");

        let buf: [u8; 4] = [(data >> 24) as u8, ((data >> 16) & 0xFF) as u8,
            ((data >> 8) & 0xFF) as u8, (data & 0xFF) as u8];
        file.seek_write(&buf, sector_num).expect("Could not write to the file");
    }
}