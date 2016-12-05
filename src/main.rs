use std::env;
use std::fs::File;
use std::io::prelude::*;

use mips::decode_instruction;

mod mips;

fn get_filename() -> String
{
    match env::args().nth(1) {
        Some(arg) => arg,
        None => panic!("No arguments passed")
    }
}

fn get_file_contents(filename: String) -> String
{
    let mut file = match File::open(&filename) {
        Err(why) => panic!("File could not be opened: {}", why),
        Ok(result) => result
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("File could not be read: {}", why),
        Ok(_) => print!("File contents:\n{}\n\n", contents)
    };

    contents
}

fn main()
{
    let file_contents = {
        let filename = get_filename();
        get_file_contents(filename)
    };

    let mut code_vec  = Vec::new();
    let mut instr_vec = Vec::new();
    for instr_str in file_contents.split_whitespace() {

        let instr_val = match u32::from_str_radix(instr_str, 16) {
            Ok(result) => result,
            Err(why) => panic!("Unable to parse hex: {}", why)
        };

        if instr_val == mips::TERMINATE {
            break;
        }

        code_vec.push(instr_val);

        let instr = {
            match decode_instruction(instr_val) {
                Some(i) => i,
                None => panic!("Invalid instruction: 0x{:08x}", instr_val)
            }
        };
        instr_vec.push(instr);
    }

    let mut curr_addr = mips::START_ADDR;
    for (instr, code) in instr_vec.iter().zip(code_vec.iter()) {
        println!("[0x{:08x}] 0x{:08x} {}", curr_addr, code, instr);
        curr_addr += mips::BYTE_SIZE as u32;
    }
}
