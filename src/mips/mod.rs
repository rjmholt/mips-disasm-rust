mod decode;
pub mod data;

use mips::data::Instr;

pub fn decode_instruction(instr: u32) -> Option<Instr>
{
    decode::decode_instruction(instr)
}

pub const TERMINATE:  u32 = data::TERMINATE;
pub const START_ADDR: u32 = data::START_ADDR;
pub const BYTE_SIZE:  u16 = data::BYTE_SIZE;
