#![feature(try_from)]

extern crate byteorder;

const IND_SIZE: usize = 2;
const REG_SIZE: usize = 4;
const DIR_SIZE: usize = REG_SIZE;

const REG_CODE: i32 = 1;
const DIR_CODE: i32 = 2;
const IND_CODE: i32 = 3;

const MAX_ARGS_NUMBER: i32 = 4;
const MAX_PLAYERS:     i32 = 4;
const MEM_SIZE:        i32 = 4 * 1024;
const IDX_MOD:         i32 = MEM_SIZE / 8;
const CHAMP_MAX_SIZE:  i32 = MEM_SIZE / 6;

const COMMENT_CHAR:    u8 = b'#';
const LABEL_CHAR:      u8 = b':';
const DIRECT_CHAR:     u8 = b'%';
const SEPARATOR_CHAR:  u8 = b',';

const LABEL_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_0123456789";

const NAME_CMD_STRING:    &str = ".name";
const COMMENT_CMD_STRING: &str = ".comment";

const REG_NUMBER: i32 = 16;

const CYCLE_TO_DIE: i32 = 1536;
const CYCLE_DELTA:  i32 = 50;
const NBR_LIVE:     i32 = 21;
const MAX_CHECKS:   i32 = 10;

// typedef char	t_arg_type;

const T_REG: u8 = 1;
const T_DIR: u8 = 2;
const T_IND: u8 = 4;
const T_LAB: u8 = 8;

mod instruction;

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

//

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
