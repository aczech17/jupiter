mod parse_args;
use parse_args::get_args;
use computer::*;
mod display;
use display::display;

fn main()
{

    let (rom_filename, program_filename,
        disk_filename, disk_size, memory_size, vram_size) =
    get_args();


    let computer = Computer::new(
        rom_filename,
        program_filename,
        disk_filename, disk_size, memory_size, vram_size);

    display(computer, 800, 600);
}
