use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use mips::Jump;

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
        Ok(_) => print!("File contents:\n{}\n", contents)
    };

    contents
}

struct Block
{
    pub label: String,
    pub instrs: Vec<mips::Instr>
}

impl std::fmt::Display for Block
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        try!(write!(f, "{}:\n", self.label));

        for ref instr in &self.instrs {
            try!(write!(f, "\t{}\n", instr));
        }

        Ok(())
    }
}

fn index_to_addr(index: usize) -> u32
{
    mips::START_ADDR + (mips::BYTE_SIZE as u32) * (index as u32)
}

fn main()
{
    let file_contents = {
        let filename = get_filename();
        get_file_contents(filename)
    };

    let mut instr_vec = match mips::string_to_instr_vec(file_contents) {
        Ok(iv) => iv,
        Err(why) => panic!(why)
    };

    mips::print_instrs(&mut io::stdout(), &instr_vec);

    let mut curr_addr = mips::START_ADDR;
    let mut branches = vec![0x00400024];
    for &(_, ref instr) in &instr_vec {
        match instr {
            &mips::Instr::J(ref j) =>
                branches.push(j.branch_addr(curr_addr)),
            &mips::Instr::JaL(ref jal) =>
                branches.push(jal.branch_addr(curr_addr)),
            &mips::Instr::BGTZ(ref bgtz) => {
                branches.push(bgtz.branch_addr(curr_addr))
            },
            _ => ()
        }
        curr_addr += mips::BYTE_SIZE as u32;
    }

    branches.sort();

    for &addr in &branches {
        println!("0x{:08x}", addr)
    }

    let main_block = Block {
        label: "main".to_string(),
        instrs: Vec::new()
    };

    let mut blocks = vec![main_block];
    curr_addr = mips::START_ADDR;

    for &addr in &branches {
        blocks.push(Block {
            label: format!("a{:08x}", addr),
            instrs: Vec::new()
        });
    }

    // TODO: Look through the instructions and find the basic blocks
    // by finding all the addresses jumped to and then creating
    // blocks for each, plus the start address and adding the
    // component instructions to the relevant block

    let mut i: usize = 0;
    let mut j: usize = 0;
    while index_to_addr(i) < 0x00400024 {
        match instr_vec.pop() {
            Some(instr) => blocks[j].instrs.push(instr.1),
            None => () // Should break...
        }
        i += 1;
    }

    while i < instr_vec.len() {
        if index_to_addr(i) <= branches[j] {
            j += 1
        }
        match instr_vec.pop() {
            Some(instr) => blocks[j].instrs.push(instr.1),
            None => ()
        }
        i += 1;
    }
}
