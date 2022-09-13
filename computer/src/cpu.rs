use crate::cpu_aux::TransferType;
use crate::cpu_aux::TransferType::*;
use crate::cpu_aux::Phase;
use crate::cpu_aux::Phase::*;

pub(crate) struct CPU
{
    #[allow(unused)]
    fp_reg: [f32; 32],

    reg: [i32; 32],
    hi: i32,
    lo: i32,
    pc: u32,
    // REGISTERS
    // 0 - 31
    // pc - 32
    // hi - 33
    // lo - 34

    instruction: u32,
    result: i32,
    target: u8, // 5 bits
    in_out: (TransferType, u32, u32), // transfer type, address, data
    phase: Phase,
}

impl CPU
{
    pub(crate) fn new() -> Self
    {
        CPU
        {
            reg: [0; 32],
            fp_reg: [0.0; 32],
            hi: 0,
            lo: 0,
            pc: 0,
            instruction: 0,
            result: 0,
            target: 0,
            in_out: (NoTransfer, 0, 0),
            phase: IF
        }
    }
    fn next_phase(&mut self)
    {
        self.phase = match self.phase
        {
            IF => DEXE,
            DEXE => MEM,
            MEM => WB,
            WB => IF,
        };
    }

    fn write_to_reg(&mut self, num: u8, data: i32)
    {
        match num
        {
            0 => return, // 0 registers is constant 0
            32 => {
                if ((data as u32) % 4) != 0
                {
                    panic!("Instruction address not aligned");
                }
                self.pc = data as u32
            },
            33 => self.hi = data,
            34 => self.lo = data,
            n => self.reg[n as usize] = data,
        };
    }

    pub(crate) fn tick(&mut self, data: u32) -> (TransferType, u32, u32)
    {
        self.in_out.2 = data; // data read from the bus
        match self.phase
        {
            IF => self.fetch(),
            DEXE => self.decode_and_execute(),
            MEM => self.read_from_memory(),
            WB => self.write_back(),
        };

        self.next_phase();
        return self.in_out;
    }

    fn fetch(&mut self)
    {
        let (transfer_type, address, data) = (ReadWord, self.pc, 0);
        self.in_out = (transfer_type, address, data);
    }

    fn decode_and_execute(&mut self)
    {
        self.instruction = self.in_out.2;
        self.pc += 4;

        let opcode = (self.instruction & 0b_111111_00000_00000_00000_00000_000000) >> 26;

        // R
        let rs = ((self.instruction &     0b_000000_11111_00000_00000_00000_000000) >> 21) as u8;
        let rt = ((self.instruction &     0b_000000_00000_11111_00000_00000_000000) >> 16) as u8;
        let rd = ((self.instruction &     0b_000000_00000_00000_11111_00000_000000) >> 11) as u8;
        let shift = ((self.instruction &  0b_000000_00000_00000_00000_11111_000000) >> 6) as u8;
        let funct = (self.instruction &  0b_000000_00000_00000_00000_00000_111111) as u8;

        // I
        let imm = (self.instruction & 0xFFFF) as u16 as i16;

        // J
        let address = self.instruction & 0x_00_03_FF_FF; // 26 youngest bits

        #[cfg(debug_assertions)]
        println!("Instruction: {:#032b}", self.instruction);
        #[cfg(debug_assertions)]
        println!("Opcode: {}, Address: {}", opcode, address);

        if opcode == 0
        {
            match funct
            {
                0 => self.sll(rd, rt, shift),
                2 => self.srl(rd, rt, shift),
                3 => self.sra(rd, rt, shift),
                4 => self.sllv(rd, rt, rs),
                6 => self.srlv(rd, rt, rs),
                7 => self.srav(rd, rt, rs),
                8 => self.jr(rs),
                9 => self.jalr(rd, rs),
                12 => self.syscall(),
                16 => self.mfhi(rd),
                17 => self.mthi(rs),
                18 => self.mflo(rd),
                19 => self.mtlo(rs),
                24 => self.mult(rs, rt),
                25 => self.multu(rs, rt),
                26 => self.div(rs, rt),
                27 => self.divu(rs, rt),
                32 => self.add(rd, rs, rt),
                33 => self.addu(rd, rs, rt),
                34 => self.sub(rd, rs, rt),
                35 => self.subu(rd, rs, rt),
                36 => self.and(rd, rs, rt),
                37 => self.or(rd, rs, rt),
                38 => self.xor(rd, rs, rt),
                39 => self.nor(rd, rs, rt),
                42 => self.slt(rd, rs, rt),
                43 => self.sltu(rd, rs, rt),
                _ => panic!("Bad instruction at {}", self.pc - 4),
            }
        }
        else
        {
            match opcode
            {
                2 => self.j(address),
                3 => self.jal(address),
                4 => self.beq(rs, rt, imm),
                5 => self.bne(rs, rt, imm),
                6 => self.blez(rs, imm),
                7 => self.bgtz(rs, imm),
                8 => self.addi(rt, rs, imm),
                9 => self.addiu(rt, rs, imm),
                10 => self.slti(rt, rs, imm),
                11 => self.sltiu(rt, rs, imm),
                12 => self.andi(rt, rs, imm),
                13 => self.ori(rt, rs, imm),
                14 => self.xori(rt, rs, imm),
                15 => self.lui(rt, imm),
                32 => self.lb(rt, rs, imm),
                33 => self.lh(rt, rs, imm),
                34 => self.lw(rt, rs, imm),
                36 => self.lbu(rt, rs, imm),
                37 => self.lhu(rt, rs, imm),
                40 => self.sb(rt, rs, imm),
                41 => self.sh(rt, rs, imm),
                43 => self.sw(rt, rs, imm),
                _ => panic!("Bad instruction at {}", self.pc - 4),
            }
        }
    }

    fn read_from_memory(&mut self)
    {
        // target is already filled
        match self.in_out.0
        {
            NoTransfer => {},
            ReadByte => {
                let data = self.in_out.2;
                let data = data as u8 as i8 as i32; // sign extension
                self.result = data;
            }
            ReadHalf => {
                let data = self.in_out.2;
                let data = data as u8 as i8 as i32; // sign extension
                self.result = data;
            }
            ReadWord => {
                let data = self.in_out.2;
                self.result = data as i32;
            }
            ReadByteUnsigned => {
                let data = self.in_out.2;
                let data = data & 0xFF;
                self.result = data as i32;
            }
            ReadHalfUnsigned => {
                let data = self.in_out.2;
                let data = data & 0xFFFF;
                self.result = data as i32;
            }
            WriteByte => {}
            WriteHalf => {}
            WriteWord => {}
        }
        // end of transmission with memory in this cycle
        self.in_out.0 = NoTransfer;
    }

    fn write_back(&mut self)
    {
        self.write_to_reg(self.target, self.result);

        // end of cycle
        self.target = 0;
    }
}

impl CPU // opcodes
{
    fn sll(&mut self, rd: u8, rt: u8, shift: u8)
    {
        let res = self.reg[rt as usize] << shift;
        self.write_to_reg(rd, res);
    }

    fn srl(&mut self, rd: u8, rt: u8, shift: u8)
    {
        let res = (self.reg[rt as usize] as u32) >> shift;
        self.write_to_reg(rd, res as i32);
    }

    fn sra(&mut self, rd: u8, rt: u8, shift: u8)
    {
        let res = self.reg[rt as usize] >> shift;
        self.write_to_reg(rd, res);
    }

    fn sllv(&mut self, rd: u8, rt: u8, rs: u8)
    {
        let res = self.reg[rt as usize] << self.reg[rs as usize];
        self.write_to_reg(rd, res);
    }

    fn srlv(&mut self, rd: u8, rt: u8, rs: u8)
    {
        let res = (self.reg[rt as usize] as u32) >> self.reg[rs as usize];
        self.write_to_reg(rd, res as i32);
    }

    fn srav(&mut self, rd: u8, rt: u8, rs: u8)
    {
        let res = self.reg[rt as usize] >> self.reg[rs as usize];
        self.write_to_reg(rd, res);
    }

    fn jr(&mut self, rs: u8)
    {
        self.write_to_reg(32, self.reg[rs as usize]);
    }

    fn jalr(&mut self, rd: u8, rs: u8)
    {
        self.write_to_reg(rd, self.pc as i32); // save pc to rd
        self.write_to_reg(32, self.reg[rs as usize]); // jump to rs
    }

    fn syscall(&mut self)
    {
        // ???
    }

    fn mfhi(&mut self, rd: u8)
    {
        self.write_to_reg(rd, self.hi);
    }

    fn mthi(&mut self, rs: u8)
    {
        self.write_to_reg(33, self.reg[rs as usize]);
    }

    fn mflo(&mut self, rd: u8)
    {
        self.write_to_reg(rd, self.lo);
    }

    fn mtlo(&mut self, rs: u8)
    {
        self.write_to_reg(34, self.reg[rs as usize]);
    }

    fn mult(&mut self, rs: u8, rt: u8)
    {
        let left: i64 = self.reg[rs as usize] as i64;
        let right: i64 = self.reg[rt as usize] as i64;

        let result = left * right;
        let hi = ((result as u64) >> 32) as u32 as i32;
        let lo = (result & 0xFFFFFFFF) as i32;

        self.write_to_reg(33, hi);
        self.write_to_reg(34, lo);
    }

    fn multu(&mut self, rs: u8, rt: u8)
    {
        let left: u64 = self.reg[rs as usize] as u64;
        let right: u64 = self.reg[rt as usize] as u64;

        let result = left * right;
        let hi = (result >> 32) as u32 as i32;
        let lo = (result & 0xFFFFFFFF) as u32 as i32;

        self.write_to_reg(33, hi);
        self.write_to_reg(34, lo);
    }

    fn div(&mut self, rs: u8, rt: u8)
    {
        let left = self.reg[rs as usize];
        let right = self.reg[rt as usize];

        let lo = left / right;
        let hi = left % right;

        self.write_to_reg(33, hi);
        self.write_to_reg(34, lo);
    }

    fn divu(&mut self, rs: u8, rt: u8)
    {
        let left = self.reg[rs as usize] as u32;
        let right = self.reg[rt as usize] as u32;

        let lo = left / right;
        let hi = left % right;

        self.write_to_reg(33, hi as i32);
        self.write_to_reg(34, lo as i32);
    }

    fn add(&mut self, rd: u8, rs: u8, rt: u8)
    {
        let res = self.reg[rs as usize] + self.reg[rt as usize];
        self.write_to_reg(rd, res);
    }

    fn addu(&mut self, rd: u8, rs: u8, rt: u8)
    {
        let res = self.reg[rs as usize] as u32 + self.reg[rt as usize] as u32;
        self.write_to_reg(rd, res as i32);
    }

    fn sub(&mut self, rd: u8, rs: u8, rt: u8)
    {
        let res = self.reg[rs as usize] - self.reg[rt as usize];
        self.write_to_reg(rd, res);
    }

    fn subu(&mut self, rd: u8, rs: u8, rt: u8)
    {
        let res = self.reg[rs as usize] as u32 - self.reg[rt as usize] as u32;
        self.write_to_reg(rd, res as i32);
    }

    fn and(&mut self, rd: u8, rs: u8, rt: u8)
    {
        let res = self.reg[rs as usize] & self.reg[rt as usize];
        self.write_to_reg(rd, res);
    }

    fn or(&mut self, rd: u8, rs: u8, rt: u8)
    {
        let res = self.reg[rs as usize] | self.reg[rt as usize];
        self.write_to_reg(rd, res);
    }

    fn xor(&mut self, rd: u8, rs: u8, rt: u8)
    {
        let res = self.reg[rs as usize] ^ self.reg[rt as usize];
        self.write_to_reg(rd, res);
    }

    fn nor(&mut self, rd: u8, rs: u8, rt: u8)
    {
        let res = !(self.reg[rs as usize] | self.reg[rt as usize]);
        self.write_to_reg(rd, res);
    }

    fn slt(&mut self, rd: u8, rs: u8, rt: u8)
    {
        if self.reg[rs as usize] < self.reg[rt as usize]
        {
            self.write_to_reg(rd, 1);
        } else {
            self.write_to_reg(rd, 0);
        }
    }

    fn sltu(&mut self, rd: u8, rs: u8, rt: u8)
    {
        if (self.reg[rs as usize] as u32) < (self.reg[rt as usize] as u32)
        {
            self.write_to_reg(rd, 1);
        } else {
            self.write_to_reg(rd, 0);
        }
    }


    fn j(&mut self, address: u32)
    {
        let effective_address = (address << 2) | (self.pc & (0b1111 << 28));
        self.write_to_reg(32, effective_address as i32); // jump to address
    }

    fn jal(&mut self, address: u32)
    {
        self.write_to_reg(31, self.pc as i32);
        self.j(address);
    }


    fn beq(&mut self, rs: u8, rt: u8, imm: i16)
    {
        if self.reg[rs as usize] == self.reg[rt as usize]
        {
            let address = self.pc as i32 + (imm as i32);
            self.write_to_reg(32, address as i32);
        }
    }

    fn bne(&mut self, rs: u8, rt: u8, imm: i16)
    {
        if self.reg[rs as usize] != self.reg[rt as usize]
        {
            let address = self.pc as i32 + (imm as i32);
            self.write_to_reg(32, address as i32);
        }
    }

    fn blez(&mut self, rs: u8, imm: i16)
    {
        if self.reg[rs as usize] <= 0
        {
            let address = self.pc as i32 + (imm as i32);
            self.write_to_reg(32, address as i32);
        }
    }

    fn bgtz(&mut self, rs: u8, imm: i16)
    {
        if self.reg[rs as usize] > 0
        {
            let address = self.pc as i32 + (imm as i32);
            self.write_to_reg(32, address as i32);
        }
    }

    fn addi(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let res = self.reg[rs as usize] + (imm as i32);
        self.write_to_reg(rt, res);
    }

    fn addiu(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let res = (self.reg[rs as usize] as u32) + (imm as i32 as u32);
        self.write_to_reg(rt, res as i32);
    }

    fn slti(&mut self, rt: u8, rs: u8, imm: i16)
    {
        if self.reg[rs as usize] < imm as i32
        {
            self.write_to_reg(rt, 1);
        } else {
            self.write_to_reg(rt, 0);
        }
    }

    fn sltiu(&mut self, rt: u8, rs: u8, imm: i16)
    {
        if (self.reg[rs as usize] as u32) < (imm as i32 as u32)
        {
            self.write_to_reg(rt, 1)
        }
        else {
            self.write_to_reg(rt, 0);
        }
    }

    fn andi(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let res = self.reg[rs as usize] & (imm as u16 as u32 as i32);
        self.write_to_reg(rt, res);
    }

    fn ori(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let res = self.reg[rs as usize] | (imm as u16 as u32 as i32);
        self.write_to_reg(rt, res);
    }

    fn xori(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let res = self.reg[rs as usize] ^ (imm as u16 as u32 as i32);
        self.write_to_reg(rt, res);
    }

    fn lui(&mut self, rt: u8, imm: i16)
    {
        let result = (imm as u16 as u32) << 16;
        self.write_to_reg(rt, result as i32);
    }

    fn lb(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let address = self.reg[rs as usize] + (imm as i32);

        self.target = rt;
        self.in_out = (ReadByte, address as u32, 0);
    }

    fn lh(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let address = self.reg[rs as usize] + (imm as i32);

        self.target = rt;
        self.in_out = (ReadHalf, address as u32, 0);
    }

    fn lw(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let address = self.reg[rs as usize] + (imm as i32);

        self.target = rt;
        self.in_out = (ReadWord, address as u32, 0);
    }

    fn lbu(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let address = self.reg[rs as usize] + (imm as i32);

        self.target = rt;
        self.in_out = (ReadByteUnsigned, address as u32, 0);
    }

    fn lhu(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let address = self.reg[rs as usize] + (imm as i32);

        self.target = rt;
        self.in_out = (ReadHalfUnsigned, address as u32, 0);
    }

    fn sb(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let data = self.reg[rt as usize] & 0xFF;
        let address = self.reg[rs as usize] + (imm as i32);

        self.in_out = (WriteByte, address as u32, data as u32);
    }

    fn sh(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let data = self.reg[rt as usize] & 0xFFFF;
        let address = self.reg[rs as usize] + (imm as i32);

        self.in_out = (WriteHalf, address as u32, data as u32);
    }

    fn sw(&mut self, rt: u8, rs: u8, imm: i16)
    {
        let data = self.reg[rt as usize];
        let address = self.reg[rs as usize] + (imm as i32);

        self.in_out = (WriteWord, address as u32, data as u32);
    }
}

impl CPU // dump
{
    #[allow(unused)]
    fn registers(&self) -> ([i32; 35], [f32; 32])
    {
        let mut int_registers = [0; 35];
        for i in 0..32
        {
            int_registers[i] = self.reg[i];
        }
        int_registers[32] = self.pc as i32;
        int_registers[33] = self.hi as i32;
        int_registers[34] = self.lo as i32;

        let mut float_registers = [0.0; 32];
        for i in 0..32
        {
            float_registers[i] = self.fp_reg[i];
        }

        return (int_registers, float_registers);
    }
}
