use std;
use mips;
use std::fmt::Error;
use std::result::Result;

pub fn string_to_instr_vec(s: String) -> Result<Vec<(u32, mips::Instr)>, String>
{
    let mut iv = Vec::new();

    for instr_str in s.split_whitespace() {
        let instr_val = match u32::from_str_radix(instr_str, 16) {
            Ok(result) => result,
            Err(why) => return Err(why.to_string())
        };

        if instr_val == mips::TERMINATE {
            return Ok(iv)
        }

        let instr = match mips::decode::decode_instruction(instr_val) {
            Ok(i) => i,
            Err(why) =>
                return Err(why)
        };

        iv.push((instr_val, instr));
    }

    Ok(iv)
}
