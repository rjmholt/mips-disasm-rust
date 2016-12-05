use std::fmt;

// Instruction bit field lengths
pub const INSTR_LEN:  u8 = 32;
pub const OPCODE_LEN: u8 = 6;
pub const RS_LEN:     u8 = 5;
pub const RT_LEN:     u8 = 5;
pub const RD_LEN:     u8 = 5;
pub const SHAMT_LEN:  u8 = 5;
pub const FUNCT_LEN:  u8 = 6;
pub const IMM_LEN:    u8 = 16;
pub const ADDR_LEN:   u8 = 26;

// Instruction field offsets
pub const OPCODE_OFFSET: u8 = INSTR_LEN - OPCODE_LEN;
pub const RS_OFFSET:     u8 = OPCODE_OFFSET - RS_LEN;
pub const RT_OFFSET:     u8 = RS_OFFSET - RT_LEN;
pub const RD_OFFSET:     u8 = RT_OFFSET - RD_LEN;
pub const SHAMT_OFFSET:  u8 = RD_OFFSET - SHAMT_LEN;
pub const FUNCT_OFFSET:  u8 = SHAMT_OFFSET - FUNCT_LEN;
pub const IMM_OFFSET:    u8 = RT_OFFSET - IMM_LEN;
pub const ADDR_OFFSET:   u8 = OPCODE_OFFSET - ADDR_LEN;

// Instuction Opcodes
pub const P_ADDI:  u8 = 0x8;
pub const P_ADDIU: u8 = 0x9;
pub const P_ANDI:  u8 = 0xc;
pub const P_BEQ:   u8 = 0x4;
pub const P_BGTZ:  u8 = 0x7;
pub const P_BLEZ:  u8 = 0x6;
pub const P_BNE:   u8 = 0x5;
pub const P_J:     u8 = 0x2;
pub const P_JAL:   u8 = 0x3;
pub const P_LB:    u8 = 0x20;
pub const P_LUI:   u8 = 0xf;
pub const P_LW:    u8 = 0x23;
pub const P_MUL:   u8 = 0x1c;
pub const P_ORI:   u8 = 0xd;
pub const P_SB:    u8 = 0x28;
pub const P_SW:    u8 = 0x2b;
// Opcode applies to both BGEZ and BLTZ, which rely on RD to differentiate
pub const P_B_GE_LT_Z: u8 = 0x1;
// Common r-type instructions have opcode 0x0, using FUNCT instead
pub const P_ZERO: u8 = 0x0;

// Differentiating RT values for BGEZ and BLTZ
pub const RT_BGEZ: u8 = 0x1;
pub const RT_BLTZ: u8 = 0x0;

// Instruction function codes
pub const F_ADD:     u8 = 0x20;
pub const F_ADDU:    u8 = 0x21;
pub const F_AND:     u8 = 0x24;
pub const F_JALR:    u8 = 0x9;
pub const F_JR:      u8 = 0x8;
pub const F_MUL:     u8 = 0x2;
pub const F_OR:      u8 = 0x25;
pub const F_SLT:     u8 = 0x2a;
pub const F_SRA:     u8 = 0x3;
pub const F_SUB:     u8 = 0x22;
pub const F_SUBU:    u8 = 0x23;
pub const F_SYSCALL: u8 = 0xc;
// This code is used for both SLL and NOP (which is a 0 shift...)
pub const F_ZERO:    u8 = 0x0;

pub const TERMINATE: u32 = 0xffffffff;

pub const BYTE_SIZE: u16 = 4;

pub const START_ADDR: u32 = 0x00400000;

// Instruction family listing
pub enum Instr
{
    Add(Add),
    AddI(AddI),
    AddIU(AddIU),
    AddU(AddU),
    And(And),
    AndI(AndI),
    BEq(BEq),
    BGEZ(BGEZ),
    BGTZ(BGTZ),
    BLEZ(BLEZ),
    BLTZ(BLTZ),
    BNE(BNE),
    J(J),
    JaL(JaL),
    JaLR(JaLR),
    JR(JR),
    LB(LB),
    LUI(LUI),
    LW(LW),
    Mul(Mul),
    NOp(NOp),
    Or(Or),
    OrI(OrI),
    SB(SB),
    SLL(SLL),
    SLT(SLT)
}

impl fmt::Display for Instr
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Instr::Add(ref instr)   => instr.fmt(f),
            Instr::AddI(ref instr)  => instr.fmt(f),
            Instr::AddIU(ref instr) => instr.fmt(f),
            Instr::AddU(ref instr)  => instr.fmt(f),
            Instr::And(ref instr)   => instr.fmt(f),
            Instr::AndI(ref instr)  => instr.fmt(f),
            Instr::BEq(ref instr)   => instr.fmt(f),
            Instr::BGEZ(ref instr)  => instr.fmt(f),
            Instr::BGTZ(ref instr)  => instr.fmt(f),
            Instr::BLEZ(ref instr)  => instr.fmt(f),
            Instr::BLTZ(ref instr)  => instr.fmt(f),
            Instr::BNE(ref instr)   => instr.fmt(f),
            Instr::J(ref instr)     => instr.fmt(f),
            Instr::JaL(ref instr)   => instr.fmt(f),
            Instr::JaLR(ref instr)  => instr.fmt(f),
            Instr::JR(ref instr)    => instr.fmt(f),
            Instr::LB(ref instr)    => instr.fmt(f),
            Instr::LUI(ref instr)   => instr.fmt(f),
            Instr::LW(ref instr)    => instr.fmt(f),
            Instr::Mul(ref instr)   => instr.fmt(f),
            Instr::NOp(ref instr)   => instr.fmt(f),
            Instr::Or(ref instr)    => instr.fmt(f),
            Instr::OrI(ref instr)   => instr.fmt(f),
            Instr::SB(ref instr)    => instr.fmt(f),
            Instr::SLL(ref instr)   => instr.fmt(f),
            Instr::SLT(ref instr)   => instr.fmt(f)
        }
    }
}

pub struct Add
{
    pub rs: u8,
    pub rt: u8,
    pub rd: u8
}

impl fmt::Display for Add
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "add ${}, ${}, ${}", self.rd, self.rs, self.rt)
    }
}

pub struct AddI
{
    pub rs:  u8,
    pub rt:  u8,
    pub imm: u16
}

impl fmt::Display for AddI
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "addi ${}, ${}, {}", self.rt, self.rs, self.imm)
    }
}

pub struct AddIU
{
    pub rs:  u8,
    pub rt:  u8,
    pub imm: u16
}

impl fmt::Display for AddIU
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "addiu ${}, ${}, {}", self.rt, self.rs, self.imm)
    }
}

pub struct AddU
{
    pub rs: u8,
    pub rt: u8,
    pub rd: u8
}

impl fmt::Display for AddU
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "addu ${}, ${}, ${}", self.rd, self.rs, self.rt)
    }
}

pub struct And
{
    pub rs: u8,
    pub rt: u8,
    pub rd: u8
}

impl fmt::Display for And
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "and ${}, ${}, ${}", self.rd, self.rs, self.rs)
    }
}

pub struct AndI
{
    pub rs:  u8,
    pub rt:  u8,
    pub imm: u16
}

impl fmt::Display for AndI
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "andi ${}, ${}, {}", self.rt, self.rs, self.imm)
    }
}

pub struct BEq
{
    pub rs:  u8,
    pub rt:  u8,
    pub off: u16
}

impl fmt::Display for BEq
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "beq ${}, ${}, {}", self.rs, self.rt, BYTE_SIZE * self.off)
    }
}

pub struct BGEZ
{
    pub rs:  u8,
    pub off: u16
}

impl fmt::Display for BGEZ
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "bgez ${}, {}", self.rs, BYTE_SIZE * self.off)
    }
}

pub struct BGTZ
{
    pub rs:  u8,
    pub off: u16
}

impl fmt::Display for BGTZ
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "bgtz ${}, {}", self.rs, BYTE_SIZE * self.off)
    }
}

pub struct BLEZ
{
    pub rs:  u8,
    pub off: u16
}

impl fmt::Display for BLEZ
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "blez ${}, {}", self.rs, BYTE_SIZE * self.off)
    }
}

pub struct BLTZ
{
    pub rs:  u8,
    pub off: u16
}

impl fmt::Display for BLTZ
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "bltz ${}, {}", self.rs, BYTE_SIZE * self.off)
    }
}

pub struct BNE
{
    pub rs:  u8,
    pub rt:  u8,
    pub off: u16
}

impl fmt::Display for BNE
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "bne ${}, ${}, {}", self.rs, self.rt, BYTE_SIZE * self.off)
    }
}

pub struct J
{
    pub addr: u32
}

impl fmt::Display for J
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "j 0x{:08x}", (BYTE_SIZE as u32) * self.addr)
    }
}

pub struct JaL
{
    pub addr: u32
}

impl fmt::Display for JaL
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "jal 0x{:08x}", (BYTE_SIZE as u32) * self.addr)
    }
}

pub struct JaLR
{
    pub rs: u8,
    pub rd: u8
}

impl fmt::Display for JaLR
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "jalr ${}, ${}", self.rd, self.rs)
    }
}

pub struct JR
{
    pub rs: u8
}

impl fmt::Display for JR
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "jr ${}", self.rs)
    }
}

pub struct LB
{
    pub rs:  u8,
    pub rt:  u8,
    pub off: u16
}

impl fmt::Display for LB
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "lb ${}, {}(${})", self.rt, BYTE_SIZE * self.off, self.rs)
    }
}

pub struct LUI
{
    pub rt:  u8,
    pub imm: u16
}

impl fmt::Display for LUI
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "lui ${}, {}", self.rt, BYTE_SIZE * self.imm)
    }
}

pub struct LW
{
    pub rs:  u8,
    pub rt:  u8,
    pub off: u16
}

impl fmt::Display for LW
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "lw ${}, {}(${})", self.rt, BYTE_SIZE * self.off, self.rs)
    }
}

pub struct Mul
{
    pub rs: u8,
    pub rt: u8,
    pub rd: u8
}

impl fmt::Display for Mul
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "mul ${}, ${}, ${}", self.rd, self.rs, self.rt)
    }
}

pub struct NOp {}

impl fmt::Display for NOp
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "nop")
    }
}

pub struct Or
{
    pub rs: u8,
    pub rt: u8,
    pub rd: u8
}

impl fmt::Display for Or
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "or ${}, ${}, ${}", self.rd, self.rs, self.rt)
    }
}

pub struct OrI
{
    pub rs:  u8,
    pub rt:  u8,
    pub imm: u16
}

impl fmt::Display for OrI
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "ori ${}, ${}, {}", self.rt, self.rs, self.imm)
    }
}

pub struct SB
{
    pub rs:  u8,
    pub rt:  u8,
    pub off: u16
}

impl fmt::Display for SB
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "sb ${}, {}(${})", self.rt, BYTE_SIZE * self.off, self.rs)
    }
}

pub struct SLL
{
    pub rt:    u8,
    pub rd:    u8,
    pub shamt: u8
}

impl fmt::Display for SLL
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "sll ${}, ${}, {}", self.rd, self.rt, self.shamt)
    }
}

pub struct SLT
{
    pub rt: u8,
    pub rs: u8,
    pub rd: u8
}

impl fmt::Display for SLT
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "slt ${}, ${}, ${}", self.rd, self.rs, self.rt)
    }
}
