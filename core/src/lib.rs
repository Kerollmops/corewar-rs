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

pub const PROG_NAME_LENGTH:     usize = 128;
pub const COMMENT_LENGTH:       usize = 2048;
pub const COREWAR_EXEC_MAGIC:     u32 = 0xea83f3;

#[repr(C)]
pub struct Header {
    pub magic: u32,
    pub prog_name: [u8; PROG_NAME_LENGTH + 1],
    pub prog_size: u32,
    pub comment: [u8; COMMENT_LENGTH + 1],
}
