use std::result::Result;
use mips::data::*;

pub fn decode_instruction(x: u32) -> Result<Instr, String>
{
    let opcode = get_opcode(x);

    match opcode {
        P_ZERO => decode_zero_prefix_instruction(x),

        P_ADDI => {
            let (rs, rt, imm) = get_itype_fields(x);
            Ok(Instr::AddI(AddI { rs: rs, rt: rt, imm: imm }))
        },

        P_ADDIU => {
            let (rs, rt, imm) = get_itype_fields(x);
            Ok(Instr::AddIU(AddIU { rs: rs, rt: rt, imm: imm }))
        },

        P_ANDI => {
            let (rs, rt, imm) = get_itype_fields(x);
            Ok(Instr::AndI(AndI { rs: rs, rt: rt, imm: imm }))
        },

        P_BEQ => {
            let (rs, rt, off) = get_itype_fields(x);
            Ok(Instr::BEq(BEq { rs: rs, rt: rt, off: off }))
        },

        P_BGTZ => {
            let (rs, _, off) = get_itype_fields(x);
            Ok(Instr::BGTZ(BGTZ { rs: rs, off: off }))
        },

        P_BLEZ => {
            let (rs, _, off) = get_itype_fields(x);
            Ok(Instr::BLEZ(BLEZ { rs: rs, off: off }))
        },

        P_BNE => {
            let (rs, rt, off) = get_itype_fields(x);
            Ok(Instr::BNE(BNE { rs: rs, rt: rt, off: off }))
        },

        P_J => {
            let addr = get_addr(x);
            Ok(Instr::J(J { addr: addr }))
        },

        P_JAL => {
            let addr = get_addr(x);
            Ok(Instr::JaL(JaL { addr: addr }))
        },

        P_LB => {
            let (rs, rt, off) = get_itype_fields(x);
            Ok(Instr::LB(LB { rs: rs, rt: rt, off: off }))
        },

        P_LUI => {
            let (_, rt, imm) = get_itype_fields(x);
            Ok(Instr::LUI(LUI { rt: rt, imm: imm }))
        },

        P_LW => {
            let (rs, rt, off) = get_itype_fields(x);
            Ok(Instr::LW(LW { rs: rs, rt: rt, off: off }))
        },

        P_ORI => {
            let (rs, rt, imm) = get_itype_fields(x);
            Ok(Instr::OrI(OrI { rs: rs, rt: rt, imm: imm }))
        },

        P_SB => {
            let (rs, rt, off) = get_itype_fields(x);
            Ok(Instr::SB(SB { rs: rs, rt: rt, off: off }))
        },

        P_SW => {
            let (rs, rt, off) = get_itype_fields(x);
            Ok(Instr::SW(SW { rs: rs, rt: rt, off: off }))
        },

        P_XORI => {
            let (rs, rt, imm) = get_itype_fields(x);
            Ok(Instr::XOrI(XOrI { rs: rs, rt: rt, imm: imm }))
        },

        P_MUL => {
            let (rs, rt, rd, _, funct) = get_rtype_fields(x);
            match funct {
                F_MUL => Ok(Instr::Mul(Mul { rs: rs, rt: rt, rd: rd })),

                _ =>
                    Err(unknown_instruction("Mul opcode has bad funct field", x))
            }
        },

        P_B_GE_LT_Z => {
            let (rs, rt, off) = get_itype_fields(x);
            match rt {
                RT_BGEZ => Ok(Instr::BGEZ(BGEZ { rs: rs, off: off })),

                RT_BLTZ => Ok(Instr::BLTZ(BLTZ { rs: rs, off: off })),

                _ =>
                    Err(unknown_instruction("BGEZ/BLTZ opcode has bad rt field", x))
            }
        },

        _ => {
            Err(unknown_instruction("Unknown opcode", x))
        }
    }
}

fn decode_zero_prefix_instruction(x: u32) -> Result<Instr, String>
{
    let (rs, rt, rd, shamt, funct) = get_rtype_fields(x);

    match funct {
        F_ADD     => Ok(Instr::Add(Add { rs: rs, rt: rt, rd: rd })),

        F_ADDU    => Ok(Instr::AddU(AddU { rs: rs, rt: rt, rd: rd })),

        F_AND     => Ok(Instr::And(And { rs: rs, rt: rt, rd: rd })),

        F_JALR    => Ok(Instr::JaLR(JaLR { rs: rs, rd: rd })),

        F_JR      => Ok(Instr::JR(JR { rs: rs })),

        F_OR      => Ok(Instr::Or(Or { rs: rs, rt: rt, rd: rd })),

        F_SLT     => Ok(Instr::SLT(SLT { rs: rs, rt: rt, rd: rd })),

        F_SRA     => Ok(Instr::SRA(SRA { rt: rt, rd: rd, shamt: shamt })),

        F_SUB     => Ok(Instr::Sub(Sub { rs: rs, rt: rt, rd: rd })),

        F_SUBU    => Ok(Instr::SubU(SubU { rs: rs, rt: rt, rd: rd })),

        F_SYSCALL => Ok(Instr::Syscall(Syscall {})),

        F_XOR     => Ok(Instr::XOr(XOr { rs: rs, rt: rt, rd: rd })),

        F_ZERO =>
            if rs != 0 || rt != 0 || rd != 0 || shamt != 0 {
                Ok(Instr::SLL(SLL { rt: rt, rd: rd, shamt: shamt }))
            }
            else {
                Ok(Instr::NOp(NOp {}))
            },

        _ => {
            Err(unknown_instruction("Unknown funct code", x))
        }
    }
}

fn mask(x: u32, offset: u8, bits: u8) -> u32
{
    (x >> offset) & ((1 << bits) - 1)
}

fn get_opcode(x: u32) -> u8
{
    mask(x, OPCODE_OFFSET, OPCODE_LEN) as u8
}

fn get_rs(x: u32) -> u8
{
    mask(x, RS_OFFSET, RS_LEN) as u8
}

fn get_rt(x: u32) -> u8
{
    mask(x, RT_OFFSET, RT_LEN) as u8
}

fn get_rd(x: u32) -> u8
{
    mask(x, RD_OFFSET, RD_LEN) as u8
}

fn get_shamt(x: u32) -> u8
{
    mask(x, SHAMT_OFFSET, SHAMT_LEN) as u8
}

fn get_funct(x: u32) -> u8
{
    mask(x, FUNCT_OFFSET, FUNCT_LEN) as u8
}

fn get_imm(x: u32) -> u16
{
    mask(x, IMM_OFFSET, IMM_LEN) as u16
}

fn get_addr(x: u32) -> u32
{
    mask(x, ADDR_OFFSET, ADDR_LEN) as u32
}

fn get_rtype_fields(x: u32) -> (u8, u8, u8, u8, u8)
{
    let rs = get_rs(x);
    let rt = get_rt(x);
    let rd = get_rd(x);
    let shamt = get_shamt(x);
    let funct = get_funct(x);

    (rs, rt, rd, shamt, funct)
}

fn get_itype_fields(x: u32) -> (u8, u8, u16)
{
    let rs = get_rs(x);
    let rt = get_rt(x);
    let imm = get_imm(x);

    (rs, rt, imm)
}

fn unknown_instruction(why: &'static str, x: u32) -> String
{
    let opcode = get_opcode(x);
    let funct  = get_funct(x);
    format!("Instruction decoding error:\n\
             \t{}\n\
             \tInstruction code: 0x{:08x}\n\
             \tOpcode: 0x{:x}\n\
             \tFunct: 0x{:x}\n",
            why, x, opcode, funct)
}
