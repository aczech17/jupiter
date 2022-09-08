pub(crate) enum Phase
{
    IF = 0,
    DEXE = 1,
    MEM = 2,
    WB = 3,
}

#[allow(non_camel_case_types)]
pub(crate) enum TransferType
{
    no_transfer=0,
    read_byte=1,
    read_half=2,
    read_word=3,
    read_byte_unsigned=4,
    read_half_unsigned=5,
    write_byte=6,
    write_half=7,
    write_word=8,
}
use TransferType::*;

fn tt_from_u32(num: u32) -> TransferType
{
    match num
    {
        0 => no_transfer,
        1 => read_byte,
        2 => read_half,
        3 => read_word,
        4 => read_byte_unsigned,
        5 => read_half_unsigned,
        6 => write_byte,
        7 => write_half,
        8 => write_word,
        _ => panic!("Bad transfer type"),
    }
}

impl Clone for TransferType
{
    fn clone(&self) -> Self
    {
        let num = match self
        {
            no_transfer => 0,
            read_byte => 1,
            read_half => 2,
            read_word => 3,
            read_byte_unsigned => 4,
            read_half_unsigned => 5,
            write_byte => 6,
            write_half => 7,
            write_word => 8,
        };

        tt_from_u32(num)
    }
}

impl Copy for TransferType
{

}