mod var_dir_ind;
mod var_dir_ind_reg;
mod var_dir_reg;
mod var_ind_reg;

pub use self::var_dir_ind::VarDirInd;
pub use self::var_dir_ind_reg::VarDirIndReg;
pub use self::var_dir_reg::VarDirReg;
pub use self::var_ind_reg::VarIndReg;

use ::Label;
use machine::instruction::const_mem_size::ConstMemSize;

#[derive(Debug)]
pub enum Variable<T: ConstMemSize> {
    Complete(T),
    Incomplete(Label),
}
