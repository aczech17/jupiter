// rom, program, disk name, disk size, memory size, width, height

pub(crate) fn get_args() -> (
    Option<String>, // ROM filename
    Option<String>, // program filename
    String, // disk filename
    u64, // disk size
    u32, // memory size
    u32, // width
    u32, // height
    u32, // vram size
    )
{
    let args: Vec<String> = std::env::args().collect();
    // rom, program, disk name, disk size, memory size, width, height

    if args.len() < 8
    {
        eprintln!("ROM filename, program filename, disk name, disk size, memory size, screen width, screen height");
        std::process::exit(1);
    }

    let rom_filename = if args[1].to_lowercase() == "none"
    {
        None
    }
    else
    {
        let s = args[1].clone();
        Some(s)
    };

    let program_filename = if args[2].to_lowercase() == "none"
    {
        None
    }
    else
    {
        let s = args[2].clone();
        Some(s)
    };

    let disk_filename = args[3].clone();
    let disk_size = *&args[4].parse::<u64>().unwrap();
    let memory_size = *&args[5].parse::<u32>().unwrap();
    let width = *&args[6].parse::<u32>().unwrap();
    let height = *&args[7].parse::<u32>().unwrap();

    let vram_size = 3 * width * height;

    (rom_filename, program_filename, disk_filename, disk_size, memory_size, width, height, vram_size)
}