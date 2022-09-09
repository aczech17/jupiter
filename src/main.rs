use computer::*;

fn main()
{
    let computer =
        Computer::new(Some("bios.bin"), Some("program.bin"),4 * 1024);
    computer.run();
}
