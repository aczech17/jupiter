pub struct Config
{
    rom_filename: Option<String>,
    program_filename: Option<String>,
    disk_filename: String,
    disk_size: u64,
    memory_size: u32,
    vram_size: u32,

    width: u32,
    height: u32,
}

impl Config
{
    fn parse_size(input: &String) -> Option<u64>
    {
        match input.parse::<u64>()
        {
            Ok(value) => return Some(value),
            _ => {},
        };

        let value = &input[..input.len() - 1];
        let value = value.parse::<u64>().unwrap();

        let suffix: char = input.chars().nth(input.len() - 1).unwrap();
        let suffix = match suffix
        {
            'k' | 'K' => 1024,
            'm' | 'M' => 1024 * 1024,
            'g' | 'G' => 1024 * 1024 * 1024,
            't' | 'T' => 1024 * 1024 * 1024 * 1024,
            _ => return None, // too much
        };

        return Some(value * suffix);
    }

    pub fn from_args(args: Vec<String>) -> Config
    {
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

        let disk_size = Self::parse_size(&args[4]);
        let disk_size = match disk_size
        {
            Some(size) => size,
            None => panic!("Bad disk size"),
        };

        let memory_size = Self::parse_size(&args[5]);
        let memory_size = match memory_size
        {
            Some(size) =>
                {
                    if size >= 1 << 32
                    {
                        panic!("Memory too big");
                    }
                    size as u32
                }
            None => panic!("Bad memory size"),
        };

        let width = *&args[6].parse::<u32>().unwrap();
        let height = *&args[7].parse::<u32>().unwrap();

        let vram_size = 3 * width * height;

        Config
        {
            rom_filename,
            program_filename,
            disk_filename,
            disk_size,
            memory_size,
            width,
            height,
            vram_size
        }
    }
}

impl Config // getters
{
    pub fn rom_filename(&self) -> &Option<String>
    {
        &self.rom_filename
    }
    pub fn program_filename(&self) -> &Option<String>
    {
        &self.program_filename
    }
    pub fn disk_filename(&self) -> &String
    {
        &self.disk_filename
    }
    pub fn disk_size(&self) -> u64
    {
        self.disk_size
    }
    pub fn memory_size(&self) -> u32
    {
        self.memory_size
    }
    pub fn vram_size(&self) -> u32
    {
        self.vram_size
    }
    pub fn width(&self) -> u32
    {
        self.width
    }
    pub fn height(&self) -> u32
    {
        self.height
    }
}