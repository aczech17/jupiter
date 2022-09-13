pub(crate) enum Phase
{
    IF,
    DEXE,
    MEM,
    WB,
}

#[derive(Clone, Copy)]
pub(crate) enum TransferType
{
    NoTransfer = 0,
    ReadByte = 1,
    ReadHalf = 2,
    ReadWord = 3,
    ReadByteUnsigned = 4,
    ReadHalfUnsigned = 5,
    WriteByte = 6,
    WriteHalf = 7,
    WriteWord = 8,
}