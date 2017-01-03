use std::io::Write;
use mips;
use mips::data::Instr;

pub fn print_instrs(f: &mut Write, instr_vec: &Vec<(u32, Instr)>) -> ()
{
    let mut curr_addr = mips::START_ADDR;
    for &(ref code, ref instr) in instr_vec {
        write!(f, "[0x{:08x}] 0x{:08x} {}\n", curr_addr, code, instr);
        curr_addr += mips::BYTE_SIZE as u32;
    }
    write!(f, "\n");
}
