use computer::*;

fn main()
{
    let computer = Computer::new(Some("bios.bin"),
                      Some("program.bin"),
                      4 * 1024 * 1024,
                      800 * 600 * 3);

    //let computer = Computer::new(None, None, 1024, 3);

    computer.run();
}
