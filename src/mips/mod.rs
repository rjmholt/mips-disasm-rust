mod read;
mod data;
mod decode;
mod print;

pub use mips::data::Instr as Instr;
pub use mips::data::Jump as Jump;
pub use mips::read::string_to_instr_vec as string_to_instr_vec;
pub use mips::print::print_instrs as print_instrs;

pub use mips::data::TERMINATE  as TERMINATE;
pub use mips::data::START_ADDR as START_ADDR;
pub use mips::data::BYTE_SIZE  as BYTE_SIZE;
