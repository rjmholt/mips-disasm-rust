/*
#[derive(Debug)]
pub struct AddI
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct AddU
{
    rs: u8,
    rt: u8,
    rd: u8
}

#[derive(Debug)]
pub struct AddIU
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct And
{
    rs: u8,
    rt: u8,
    rd: u8
}

#[derive(Debug)]
pub struct AndI
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct BEq
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct BGEZ
{
    rs:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct BGTZ
{
    rs:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct BLEZ
{
    rs:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct BLTZ
{
    rs:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct BNE
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct LB
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct LUI
{
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct LW
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct Mul
{
    rs: u8,
    rt: u8,
    rd: u8
}

#[derive(Debug)]
pub struct J
{
    target: u32
}

#[derive(Debug)]
pub struct JaL
{
    target: u32
}

#[derive(Debug)]
pub struct JaLR
{
    rs: u8,
    rd: u8
}

#[derive(Debug)]
pub struct JR
{
    rs: u8
}

#[derive(Debug)]
pub struct NOp {}

#[derive(Debug)]
pub struct Or
{
    rs: u8,
    rt: u8,
    rd: u8
}

#[derive(Debug)]
pub struct OrI
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct SB
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct SLL
{
    rs:    u8,
    rt:    u8,
    rd:    u8,
    shamt: u8
}

#[derive(Debug)]
pub struct SLT
{
    rs:    u8,
    rt:    u8,
    rd:    u8,
    shamt: u8
}

#[derive(Debug)]
pub struct SRA
{
    rs:    u8,
    rt:    u8,
    rd:    u8,
    shamt: u8
}

#[derive(Debug)]
pub struct Sub
{
    rs: u8,
    rt: u8,
    rd: u8
}

#[derive(Debug)]
pub struct SubU
{
    rs: u8,
    rt: u8,
    rd: u8
}

#[derive(Debug)]
pub struct SW
{
    rs:  u8,
    rt:  u8,
    imm: u16
}

#[derive(Debug)]
pub struct Syscall {}
*/

    /*
    AddI(AddI),
    AddU(AddU),
    AddIU(AddIU),
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
    SLT(SLT),
    SRA(SRA),
    Sub(Sub),
    SubU(SubU),
    SW(SW),
    Syscall(Syscall),
    */

        /*
        P_ADDI => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::AddI(AddI { rs: rs, rt: rt, imm: imm }))
        },

        P_ADDIU => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::AddIU(AddIU { rs: rs, rt: rt, imm: imm }))
        },

        P_ANDI => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::AndI(AndI { rs: rs, rt: rt, imm: imm }))
        },

        P_BEQ => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::BEq(BEq { rs: rs, rt: rt, imm: imm }))
        },

        P_BGTZ => {
            let (rs, _, imm) = get_itype_fields(x);
            Some(Instr::BGTZ(BGTZ { rs: rs, imm: imm }))
        },

        P_BLEZ => {
            let (rs, _, imm) = get_itype_fields(x);
            Some(Instr::BLEZ(BLEZ { rs: rs, imm: imm }))
        },

        P_BNE => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::BNE(BNE { rs: rs, rt: rt, imm: imm }))
        },

        P_J => {
            let addr = get_addr(x);
            Some(Instr::J(J { target: addr }))
        },

        P_JAL => {
            let addr = get_addr(x);
            Some(Instr::JaL(JaL { target: addr }))
        },

        P_LB => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::LB(LB { rs: rs, rt: rt, imm: imm }))
        },

        P_LUI => {
            let (_, rt, imm) = get_itype_fields(x);
            Some(Instr::LUI(LUI { rt: rt, imm: imm }))
        },

        P_LW => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::LW(LW { rs: rs, rt: rt, imm: imm }))
        },

        P_MUL => {
            let (rs, rt, rd, _, funct) = get_rtype_fields(x);
            if funct == F_MUL {
                Some(Instr::Mul(Mul { rs: rs, rt: rt, rd: rd }))
            }
            else {
                unknown_instruction(x)
            }
        },

        P_ORI => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::OrI(OrI { rs: rs, rt: rt, imm: imm }))
        },

        P_SB => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::SB(SB { rs: rs, rt: rt, imm: imm }))
        },

        P_SW => {
            let (rs, rt, imm) = get_itype_fields(x);
            Some(Instr::SW(SW { rs: rs, rt: rt, imm: imm }))
        },

        P_B_GE_LT_Z => {
            let (rs, rt, imm) = get_itype_fields(x);

            if rt == RT_BGEZ {
                Some(Instr::BGEZ(BGEZ { rs: rs, imm: imm }))
            }
            else if rt == RT_BLTZ {
                Some(Instr::BLTZ(BLTZ { rs: rs, imm: imm }))
            }
            else {
                unknown_instruction(x)
            }
        },
        */


        /*
        F_ADDU => Some(Instr::AddU(AddU { rs: rs, rt: rt, rd: rd })),

        F_AND => Some(Instr::And(And { rs: rs, rt: rt, rd: rd })),

        F_JALR => Some(Instr::JaLR(JaLR { rs: rs, rd: rd })),

        F_JR => Some(Instr::JR(JR { rs: rs })),

        F_OR => Some(Instr::Or(Or { rs: rs, rt: rt, rd: rd })),

        F_SLT => Some(Instr::SLT(SLT { rs: rs, rt: rt, rd: rd, shamt: shamt })),

        F_SRA => Some(Instr::SRA(SRA { rs: rs, rt: rt, rd: rd, shamt: shamt })),

        F_SUB => Some(Instr::Sub(Sub { rs: rs, rt: rt, rd: rd })),

        F_SUBU => Some(Instr::SubU(SubU { rs: rs, rt: rt, rd: rd })),

        F_SYSCALL => Some(Instr::Syscall(Syscall {})),

        F_ZERO => if rs == 0x0 && rt == 0x0 && rd == 0x0 && shamt == 0x0 {
            Some(Instr::NOp(NOp {}))
        }
        else {
            Some(Instr::SLL(SLL { rs: rs, rt: rt, rd: rd, shamt: shamt }))
        },
        */

/*
impl fmt::Display for AddI
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "addi ${}, ${}, {}", self.rt, self.rs, self.imm)
    }
}

impl fmt::Display for AddU
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "addu ${}, ${}, ${}", self.rd, self.rs, self.rt)
    }
}

impl fmt::Display for AddIU
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "addiu ${}, ${}, {}", self.rt, self.rs, self.imm)
    }
}

impl fmt::Display for And
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "and ${}, ${}, ${}", self.rd, self.rs, self.rt)
    }
}

impl fmt::Display for AndI
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "andi ${}, ${}, {}", self.rt, self.rs, self.imm)
    }
}

impl fmt::Display for BEq
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "beq ${}, ${}, {}", self.rs, self.rt, BYTE_SIZE * self.imm)
    }
}

impl fmt::Display for BGEZ
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "bgez ${}, {}", self.rs, BYTE_SIZE * self.imm)
    }
}

impl fmt::Display for BGTZ
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "bgtz ${}, {}", self.rs, BYTE_SIZE * self.imm)
    }
}

impl fmt::Display for BLEZ
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "blez ${}, {}", self.rs, BYTE_SIZE * self.imm)
    }
}

impl fmt::Display for BLTZ
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "bltz ${}, {}", self.rs, BYTE_SIZE * self.imm)
    }
}

impl fmt::Display for BNE
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "bltz ${}, ${}, {}", self.rs, self.rt, BYTE_SIZE * self.imm)
    }
}

impl fmt::Display for J
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "j 0x{:08x}", START_ADDR + BYTE_SIZE * self.target)
    }
}

impl fmt::Display for JaL
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "jal 0x{:08x}", START_ADDR + BYTE_SIZE * self.target)
    }
}
*/
