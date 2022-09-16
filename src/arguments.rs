// rom, program, disk name, disk size, memory size, width, height



pub(crate) fn get_args() -> Vec<String>
{
    let args: Vec<String> = std::env::args().collect();
    // rom, program, disk name, disk size, memory size, width, height

    if args.len() < 8 && args.len() != 1
    {
        eprintln!("ROM filename, program filename, disk name, disk size, memory size, screen width, screen height");
        std::process::exit(1);
    }

   return args;
}