#![feature(try_from)]

extern crate byteorder;

pub const IND_SIZE: usize = 2;
pub const REG_SIZE: usize = 4;
pub const DIR_SIZE: usize = REG_SIZE;

pub const MAX_ARGS_NUMBER: usize = 4;
pub const MAX_PLAYERS:     usize = 4;
pub const MEM_SIZE:        usize = 4 * 1024;
pub const IDX_MOD:         usize = MEM_SIZE / 8;
pub const CHAMP_MAX_SIZE:  usize = MEM_SIZE / 6;

pub const COMMENT_CHAR:    char = '#';
pub const LABEL_CHAR:      char = ':';
pub const DIRECT_CHAR:     char = '%';
pub const SEPARATOR_CHAR:  char = ',';

pub const LABEL_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_0123456789";

pub const NAME_CMD_STRING:    &str = ".name";
pub const COMMENT_CMD_STRING: &str = ".comment";

pub const REG_NUMBER: usize = 16;
pub const REG_MAX:       u8 = REG_NUMBER as u8;

pub const CYCLE_TO_DIE: usize = 1536;
pub const CYCLE_DELTA:  usize = 50;
pub const NBR_LIVE:     usize = 21;
pub const MAX_CHECKS:   usize = 10;

// typedef char	t_arg_type;

const T_REG: u8 = 1;
const T_DIR: u8 = 2;
const T_IND: u8 = 4;
const T_LAB: u8 = 8;

mod mem_size;
mod instruction;
mod parameter;

// /*
// **
// */

// # define PROG_NAME_LENGTH		(128)
// # define COMMENT_LENGTH			(2048)
// # define COREWAR_EXEC_MAGIC		0xea83f3

// typedef struct		header_s
// {
//   unsigned int		magic;
//   char				prog_name[PROG_NAME_LENGTH + 1];
//   unsigned int		prog_size;
//   char				comment[COMMENT_LENGTH + 1];
// }					header_t;


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
