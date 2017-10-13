pub mod parameter;
pub mod mem_size;
mod get_value;
mod set_value;
mod write_to;

use std::io::{self, Read, Write};
use std::fmt;
use std::convert::TryFrom;
use byteorder::ReadBytesExt;
use instruction::mem_size::ConstMemSize;
use self::parameter::*;
use self::mem_size::MemSize;
use self::write_to::WriteTo;
use self::get_value::GetValue;
use self::set_value::SetValue;
use arena::ArenaIndex;
use machine::Machine;
use process::Context;
use core::IDX_MOD;

pub const OP_CODE_SIZE:     usize = 1;
pub const PARAM_CODE_SIZE:  usize = 1;

use self::Instruction::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    /// This instruction is the most important one,
    /// it permit to keep a process in-live also
    /// has the effect of bringing that the
    /// player whose number is in parameter is alive.
    ///
    /// It's only parameter is a `Direct`.
    Live(Direct),

    /// Loads the first parameter to the register specified as second parameter.
    ///
    /// The first parameter is a `Direct` or an `Indirect` and
    /// the second is a `Register`.
    Load(DirInd, Register),

    /// Stores the value of the first parameter in the second parameter.
    ///
    /// The first parameter is a `Register` and
    /// the second is a `Indirect` or an `Register`.
    Store(Register, IndReg),

    /// Takes three registers, sums the first two parameters and
    /// stores the result in the third.
    ///
    /// The first, second and third parameters are `Register`s.
    Addition(Register, Register, Register),

    /// Takes three registers, substract the first two parameters and
    /// stores the result in the third.
    ///
    /// The first, second and third parameters are `Register`s.
    Substraction(Register, Register, Register),

    /// Apply a binary-and (*&*) on the first two parameters and
    /// stores the value in the third parameter.
    ///
    /// The first and scond arguments are `Direct`, `Indirect` or `Register` and
    /// the third is a `Register`.
    And(DirIndReg, DirIndReg, Register),

    /// Apply a binary-or (*|*) on the first two parameters and
    /// stores the value in the third parameter.
    ///
    /// The first and scond arguments are `Direct`, `Indirect` or `Register` and
    /// the third is a `Register`.
    Or(DirIndReg, DirIndReg, Register),

    /// Apply a binary-xor (*^*) on the first two parameters and
    /// stores the value in the third parameter.
    ///
    /// The first and second arguments are of type `Direct`, `Indirect` or `Register` and
    /// the third is a `Register`.
    Xor(DirIndReg, DirIndReg, Register),

    /// Jumps to the given address if the *carry* is *true*.
    ///
    /// *warning*: It uses `alternative direct`.
    ///
    /// It's only parameter is a `Direct` value.
    ZJump(AltDirect),

    /// Sums the first two parameters and reads at this computed address and
    /// stores the value to the third parameter.
    ///
    /// *warning*: It uses `alternative direct`.
    ///
    /// The first parameter is a `Direct`, an `Indirect` or a `Register`,
    /// the second is a `Direct` or a `Register` and
    /// the third one is a `Register`.
    LoadIndex(AltDirIndReg, AltDirReg, Register),

    /// Sums the second and third parameters and
    /// stores the first argument to the computed address.
    ///
    /// *warning*: It uses `alternative direct`.
    ///
    /// The first argument is a `Register`,
    /// the second is a `Direct`, an `Indirect` or a `Register` and
    /// the last one is a `Direct` or a `Register`.
    StoreIndex(Register, AltDirIndReg, AltDirReg),

    /// Create a new process that inherit of the different states of the father.
    ///
    /// *fork pc*: _PC_ + (first argument % _IDX_MOD_)
    ///
    /// *warning*: It uses `alternative direct`.
    ///
    /// It's first argument is a `Direct`.
    Fork(AltDirect),

    /// Loads the first parameter to the register specified as second parameter.
    ///
    /// *address*: _PC_ + first parameter
    ///
    /// The first parameter is a `Direct` or an `Indirect` and
    /// the second is a `Register`.
    LongLoad(DirInd, Register),

    /// Sums the first two parameters and reads at this computed address and
    /// stores the value to the third parameter.
    ///
    /// *address*: _PC_ + first parameter
    ///
    /// *warning*: It uses `alternative direct`.
    ///
    /// The first parameter is a `Direct`, an `Indirect` or a `Register`,
    /// the second is a `Direct` or a `Register` and
    /// the third one is a `Register`.
    LongLoadIndex(AltDirIndReg, AltDirReg, Register),

    /// Create a new process that inherit of the different states of the father.
    ///
    /// *fork pc*: _PC_ + first argument
    ///
    /// *warning*: It uses `alternative direct`.
    ///
    /// It's first argument is a `Direct`.
    LongFork(AltDirect),

    /// Output the first argument % 256 to standard output.
    ///
    /// The first argument is a `Register`.
    Display(Register),
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidCode(u8),
    InvalidParamType(InvalidParamType),
    InvalidParamCode(InvalidParamCode),
    InvalidRegister(InvalidRegister),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<InvalidParamCode> for Error {
    fn from(error: InvalidParamCode) -> Self {
        Error::InvalidParamCode(error)
    }
}

impl From<DirIndRegError> for Error {
    fn from(error: DirIndRegError) -> Self {
        match error {
            DirIndRegError::Io(e) => Error::Io(e),
            DirIndRegError::InvalidRegister(invalid) => Error::InvalidRegister(invalid),
        }
    }
}

impl From<DirRegError> for Error {
    fn from(error: DirRegError) -> Self {
        match error {
            DirRegError::Io(e) => Error::Io(e),
            DirRegError::InvalidRegister(invalid) => Error::InvalidRegister(invalid),
            DirRegError::InvalidParamType(invalid) => Error::InvalidParamType(invalid),
        }
    }
}

impl From<IndRegError> for Error {
    fn from(error: IndRegError) -> Self {
        match error {
            IndRegError::Io(e) => Error::Io(e),
            IndRegError::InvalidRegister(invalid) => Error::InvalidRegister(invalid),
            IndRegError::InvalidParamType(invalid) => Error::InvalidParamType(invalid),
        }
    }
}

impl From<RegisterError> for Error {
    fn from(error: RegisterError) -> Self {
        match error {
            RegisterError::Io(e) => Error::Io(e),
            RegisterError::InvalidRegister(invalid) => Error::InvalidRegister(invalid),
        }
    }
}

impl From<DirIndError> for Error {
    fn from(error: DirIndError) -> Self {
        match error {
            DirIndError::Io(e) => Error::Io(e),
            DirIndError::InvalidParamType(invalid) => Error::InvalidParamType(invalid),
        }
    }
}

impl Instruction {
    pub const SMALLEST_INSTR_SIZE: usize = OP_CODE_SIZE + Register::MEM_SIZE;

    pub fn read_from<R: Read>(mut reader: R) -> Result<Self, Error> {
        Ok(match reader.read_u8()? {
            1 => {
                let dir = Direct::try_from(&mut reader)?;

                Live(dir)
            },
            2 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let param_type = param_code.param_type_of(ParamNumber::First)?;

                let dir_ind = DirInd::try_from((param_type, &mut reader))?;
                let reg = Register::try_from(&mut reader)?;

                Load(dir_ind, reg)
            },
            3 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let param_type = param_code.param_type_of(ParamNumber::Second)?;

                let reg = Register::try_from(&mut reader)?;
                let ind_reg = IndReg::try_from((param_type, &mut reader))?;

                Store(reg, ind_reg)
            },
            4 => {
                let reg_a = Register::try_from(&mut reader)?;
                let reg_b = Register::try_from(&mut reader)?;
                let reg_c = Register::try_from(&mut reader)?;

                Addition(reg_a, reg_b, reg_c)
            },
            5 => {
                let reg_a = Register::try_from(&mut reader)?;
                let reg_b = Register::try_from(&mut reader)?;
                let reg_c = Register::try_from(&mut reader)?;

                Substraction(reg_a, reg_b, reg_c)
            },
            6 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let first_type = param_code.param_type_of(ParamNumber::First)?;
                let second_type = param_code.param_type_of(ParamNumber::Second)?;

                let dir_ind_reg_a = DirIndReg::try_from((first_type, &mut reader))?;
                let dir_ind_reg_b = DirIndReg::try_from((second_type, &mut reader))?;
                let reg = Register::try_from(&mut reader)?;

                And(dir_ind_reg_a, dir_ind_reg_b, reg)
            },
            7 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let first_type = param_code.param_type_of(ParamNumber::First)?;
                let second_type = param_code.param_type_of(ParamNumber::Second)?;

                let dir_ind_reg_a = DirIndReg::try_from((first_type, &mut reader))?;
                let dir_ind_reg_b = DirIndReg::try_from((second_type, &mut reader))?;
                let reg = Register::try_from(&mut reader)?;

                Or(dir_ind_reg_a, dir_ind_reg_b, reg)
            },
            8 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let first_type = param_code.param_type_of(ParamNumber::First)?;
                let second_type = param_code.param_type_of(ParamNumber::Second)?;

                let dir_ind_reg_a = DirIndReg::try_from((first_type, &mut reader))?;
                let dir_ind_reg_b = DirIndReg::try_from((second_type, &mut reader))?;
                let reg = Register::try_from(&mut reader)?;

                Xor(dir_ind_reg_a, dir_ind_reg_b, reg)
            },
            9 => {
                let alt_dir = AltDirect::try_from(&mut reader)?;

                ZJump(alt_dir)
            },
            10 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let first_type = param_code.param_type_of(ParamNumber::First)?;
                let second_type = param_code.param_type_of(ParamNumber::Second)?;

                let alt_dir_ind_reg = AltDirIndReg::try_from((first_type, &mut reader))?;
                let alt_dir_reg = AltDirReg::try_from((second_type, &mut reader))?;
                let reg = Register::try_from(&mut reader)?;

                LoadIndex(alt_dir_ind_reg, alt_dir_reg, reg)
            },
            11 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let second_type = param_code.param_type_of(ParamNumber::Second)?;
                let third_type = param_code.param_type_of(ParamNumber::Third)?;

                let reg = Register::try_from(&mut reader)?;
                let alt_dir_ind_reg = AltDirIndReg::try_from((second_type, &mut reader))?;
                let alt_dir_reg = AltDirReg::try_from((third_type, &mut reader))?;

                StoreIndex(reg, alt_dir_ind_reg, alt_dir_reg)
            },
            12 => {
                let alt_dir = AltDirect::try_from(&mut reader)?;

                Fork(alt_dir)
            },
            13 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let first_type = param_code.param_type_of(ParamNumber::First)?;

                let dir_ind = DirInd::try_from((first_type, &mut reader))?;
                let reg = Register::try_from(&mut reader)?;

                LongLoad(dir_ind, reg)
            },
            14 => {
                let param_code = ParamCode::try_from(&mut reader)?;
                let first_type = param_code.param_type_of(ParamNumber::First)?;
                let second_type = param_code.param_type_of(ParamNumber::Second)?;

                let alt_dir_ind_reg = AltDirIndReg::try_from((first_type, &mut reader))?;
                let alt_dir_reg = AltDirReg::try_from((second_type, &mut reader))?;
                let reg = Register::try_from(&mut reader)?;

                LongLoadIndex(alt_dir_ind_reg, alt_dir_reg, reg)
            },
            15 => {
                let alt_dir = AltDirect::try_from(&mut reader)?;

                LongFork(alt_dir)
            }
            16 => {
                let reg = Register::try_from(&mut reader)?;

                Display(reg)
            },
            code => return Err(Error::InvalidCode(code)),
        })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write(&[self.op_code()])?;
        match *self {
            Live(dir) => {
                dir.write_to(writer)?;
            },
            Load(dir_ind, reg) => {
                ParamCode::builder()
                    .first(&dir_ind)
                    .build().write_to(writer)?;

                dir_ind.write_to(writer)?;
                reg.write_to(writer)?;
            },
            Store(reg, ind_reg) => {
                ParamCode::builder()
                    .second(&ind_reg)
                    .build().write_to(writer)?;

                reg.write_to(writer)?;
                ind_reg.write_to(writer)?;
            },
            Addition(reg_a, reg_b, reg_c) => {
                reg_a.write_to(writer)?;
                reg_b.write_to(writer)?;
                reg_c.write_to(writer)?;
            },
            Substraction(reg_a, reg_b, reg_c) => {
                reg_a.write_to(writer)?;
                reg_b.write_to(writer)?;
                reg_c.write_to(writer)?;
            },
            And(dir_ind_reg_a, dir_ind_reg_b, reg) => {
                ParamCode::builder()
                    .first(&dir_ind_reg_a).second(&dir_ind_reg_b)
                    .build().write_to(writer)?;

                dir_ind_reg_a.write_to(writer)?;
                dir_ind_reg_b.write_to(writer)?;
                reg.write_to(writer)?;
            },
            Or(dir_ind_reg_a, dir_ind_reg_b, reg) => {
                ParamCode::builder()
                    .first(&dir_ind_reg_a).second(&dir_ind_reg_b)
                    .build().write_to(writer)?;

                dir_ind_reg_a.write_to(writer)?;
                dir_ind_reg_b.write_to(writer)?;
                reg.write_to(writer)?;
            },
            Xor(dir_ind_reg_a, dir_ind_reg_b, reg) => {
                ParamCode::builder()
                    .first(&dir_ind_reg_a).second(&dir_ind_reg_b)
                    .build().write_to(writer)?;

                dir_ind_reg_a.write_to(writer)?;
                dir_ind_reg_b.write_to(writer)?;
                reg.write_to(writer)?;
            },
            ZJump(dir) => {
                dir.write_to(writer)?;
            },
            LoadIndex(dir_ind_reg, dir_reg, reg) => {
                ParamCode::builder()
                    .first(&dir_ind_reg).second(&dir_reg)
                    .build().write_to(writer)?;

                dir_ind_reg.write_to(writer)?;
                dir_reg.write_to(writer)?;
                reg.write_to(writer)?;
            },
            StoreIndex(reg, dir_ind_reg, dir_reg) => {
                ParamCode::builder()
                    .second(&dir_ind_reg).third(&dir_reg)
                    .build().write_to(writer)?;

                reg.write_to(writer)?;
                dir_ind_reg.write_to(writer)?;
                dir_reg.write_to(writer)?;
            },
            Fork(dir) => {
                dir.write_to(writer)?;
            },
            LongLoad(dir_ind, reg) => {
                ParamCode::builder()
                    .first(&dir_ind)
                    .build().write_to(writer)?;

                dir_ind.write_to(writer)?;
                reg.write_to(writer)?;
            },
            LongLoadIndex(dir_ind_reg, dir_reg, reg) => {
                ParamCode::builder()
                    .first(&dir_ind_reg).second(&dir_reg)
                    .build().write_to(writer)?;

                dir_ind_reg.write_to(writer)?;
                dir_reg.write_to(writer)?;
                reg.write_to(writer)?;
            },
            LongFork(dir) => {
                dir.write_to(writer)?;
            },
            Display(reg) => {
                reg.write_to(writer)?;
            },
        }
        Ok(())
    }

    pub fn op_code(&self) -> u8 {
        match *self {
            Live(_) => 1,
            Load(_, _) => 2,
            Store(_, _) => 3,
            Addition(_, _, _) => 4,
            Substraction(_, _, _) => 5,
            And(_, _, _) => 6,
            Or(_, _, _) => 7,
            Xor(_, _, _) => 8,
            ZJump(_) => 9,
            LoadIndex(_, _, _) => 10,
            StoreIndex(_, _, _) => 11,
            Fork(_) => 12,
            LongLoad(_, _) => 13,
            LongLoadIndex(_, _, _) => 14,
            LongFork(_) => 15,
            Display(_) => 16,
        }
    }

    pub fn cycle_cost(&self) -> usize {
        match *self {
            Live(_) => 10,
            Load(_, _) => 5,
            Store(_, _) => 5,
            Addition(_, _, _) => 10,
            Substraction(_, _, _) => 10,
            And(_, _, _) => 6,
            Or(_, _, _) => 6,
            Xor(_, _, _) => 6,
            ZJump(_) => 20,
            LoadIndex(_, _, _) => 25,
            StoreIndex(_, _, _) => 25,
            Fork(_) => 800,
            LongLoad(_, _) => 10,
            LongLoadIndex(_, _, _) => 50,
            LongFork(_) => 1000,
            Display(_) => 2,
        }
    }

    pub fn execute<W: Write>(&self, machine: &mut Machine, context: &mut Context, output: &mut W) -> io::Result<()> {
        match *self {
            Live(champion_id) => {
                context.cycle_since_last_live = 0;
                machine.live_champion(champion_id.into());
                context.pc = context.pc.advance_by(self.mem_size());
            },
            Load(dir_ind, reg) => {
                let value = dir_ind.get_value(machine, context);
                context.registers[reg] = value;
                context.carry = { value == 0 };
                context.pc = context.pc.advance_by(self.mem_size());
            },
            Store(reg, ind_reg) => {
                let value = context.registers[reg];
                match ind_reg {
                    IndReg::Indirect(ind) => ind.set_value(value, machine, context),
                    IndReg::Register(reg) => context.registers[reg] = value,
                }
                context.pc = context.pc.advance_by(self.mem_size());
            },
            Addition(reg_a, reg_b, reg_c) => {
                let val_a = context.registers[reg_a];
                let val_b = context.registers[reg_b];
                let result = val_a.wrapping_add(val_b);
                context.registers[reg_c] = result;
                context.carry = { result == 0 };
                context.pc = context.pc.advance_by(self.mem_size());
            },
            Substraction(reg_a, reg_b, reg_c) => {
                let val_a = context.registers[reg_a];
                let val_b = context.registers[reg_b];
                let result = val_a.wrapping_sub(val_b);
                context.registers[reg_c] = result;
                context.carry = { result == 0 };
                context.pc = context.pc.advance_by(self.mem_size());
            },
            And(dir_ind_reg_a, dir_ind_reg_b, reg) => {
                let val_a = dir_ind_reg_a.get_value(machine, context);
                let val_b = dir_ind_reg_b.get_value(machine, context);
                let result = val_a & val_b;
                context.registers[reg] = result;
                context.carry = { result == 0 };
                context.pc = context.pc.advance_by(self.mem_size());
            },
            Or(dir_ind_reg_a, dir_ind_reg_b, reg) => {
                let val_a = dir_ind_reg_a.get_value(machine, context);
                let val_b = dir_ind_reg_b.get_value(machine, context);
                let result = val_a | val_b;
                context.registers[reg] = result;
                context.carry = { result == 0 };
                context.pc = context.pc.advance_by(self.mem_size());
            },
            Xor(dir_ind_reg_a, dir_ind_reg_b, reg) => {
                let val_a = dir_ind_reg_a.get_value(machine, context);
                let val_b = dir_ind_reg_b.get_value(machine, context);
                let result = val_a ^ val_b;
                context.registers[reg] = result;
                context.carry = { result == 0 };
                context.pc = context.pc.advance_by(self.mem_size());
            },
            ZJump(alt_dir) => if context.carry {
                let value: i16 = alt_dir.into();
                context.pc = context.pc.move_by(value as isize % IDX_MOD as isize);
            } else {
                context.pc = context.pc.advance_by(self.mem_size());
            },
            LoadIndex(dir_ind_reg, dir_reg, reg) => {
                let val_a = dir_ind_reg.get_value(machine, context);
                let val_b = dir_reg.get_value(machine, context);
                let addr = Indirect::from(val_a.wrapping_add(val_b) as i16);
                context.registers[reg] = addr.get_value(machine, context);
                context.pc = context.pc.advance_by(self.mem_size());
            },
            StoreIndex(reg, dir_ind_reg, dir_reg) => {
                let value = context.registers[reg];
                let val_a = dir_ind_reg.get_value(machine, context);
                let val_b = dir_reg.get_value(machine, context);
                let addr = Indirect::from(val_a.wrapping_add(val_b) as i16);
                addr.set_value(value, machine, context);
                context.pc = context.pc.advance_by(self.mem_size());
            },
            Fork(alt_dir) => {
                let value: i16 = alt_dir.into();
                let mut fork = context.clean_fork();
                fork.pc = fork.pc.move_by(value as isize % IDX_MOD as isize);
                machine.new_process(fork);
                context.pc = context.pc.advance_by(self.mem_size());
            },
            LongLoad(dir_ind, reg) => {
                let value = dir_ind.get_value_long(machine, context);
                context.registers[reg] = value;
                context.carry = true; // ???
                context.pc = context.pc.advance_by(self.mem_size());
            },
            LongLoadIndex(dir_ind_reg, dir_reg, reg) => {
                let val_a = dir_ind_reg.get_value_long(machine, context);
                let val_b = dir_reg.get_value_long(machine, context);
                let addr = Indirect::from(val_a.wrapping_add(val_b) as i16);
                context.registers[reg] = addr.get_value_long(machine, context);
                context.carry = { context.pc != ArenaIndex::zero() };
                context.pc = context.pc.advance_by(self.mem_size());
            },
            LongFork(alt_dir) => {
                let value: i16 = alt_dir.into();
                let mut fork = context.clean_fork();
                fork.pc = fork.pc.move_by(value as isize);
                machine.new_process(fork);
                context.pc = context.pc.advance_by(self.mem_size());
            },
            Display(reg) => {
                let value = context.registers[reg] as u8;
                output.write_all(&[value])?;
                context.pc = context.pc.advance_by(self.mem_size());
            },
        }
        Ok(())
    }

    pub fn execute_noop(context: &mut Context) {
        context.pc = context.pc.advance_by(1)
    }
}

impl HasParamCode for Instruction {
    fn has_param_code(&self) -> bool {
        match *self {
            Live(_) => false,
            Load(_, _) => true,
            Store(_, _) => true,
            Addition(_, _, _) => false,
            Substraction(_, _, _) => false,
            And(_, _, _) => true,
            Or(_, _, _) => true,
            Xor(_, _, _) => true,
            ZJump(_) => false,
            LoadIndex(_, _, _) => true,
            StoreIndex(_, _, _) => true,
            Fork(_) => false,
            LongLoad(_, _) => true,
            LongLoadIndex(_, _, _) => true,
            LongFork(_) => false,
            Display(_) => false,
        }
    }
}

impl MemSize for Instruction {
    fn mem_size(&self) -> usize {
        let size = match *self {
            Live(a) => a.mem_size(),
            Load(a, b) => a.mem_size() + b.mem_size(),
            Store(a, b) => a.mem_size() + b.mem_size(),
            Addition(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Substraction(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            And(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Or(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Xor(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            ZJump(a) => a.mem_size(),
            LoadIndex(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            StoreIndex(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            Fork(a) => a.mem_size(),
            LongLoad(a, b) => a.mem_size() + b.mem_size(),
            LongLoadIndex(a, b, c) => a.mem_size() + b.mem_size() + c.mem_size(),
            LongFork(a) => a.mem_size(),
            Display(a) => a.mem_size(),
        };
        let param_code = if self.has_param_code() { PARAM_CODE_SIZE } else { 0 };
        OP_CODE_SIZE + param_code + size
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Live(dir) => write!(f, "live {}", dir),
            Load(dir_ind, reg) => write!(f, "ld {}, {}", dir_ind, reg),
            Store(reg, ind_reg) => write!(f, "st {}, {}", reg, ind_reg),
            Addition(reg_a, reg_b, reg_c) => write!(f, "add {}, {}, {}", reg_a, reg_b, reg_c),
            Substraction(reg_a, reg_b, reg_c) => write!(f, "sub {}, {}, {}", reg_a, reg_b, reg_c),
            And(dir_ind_reg_a, dir_ind_reg_b, reg) => write!(f, "and {}, {}, {}", dir_ind_reg_a, dir_ind_reg_b, reg),
            Or(dir_ind_reg_a, dir_ind_reg_b, reg) => write!(f, "or {}, {}, {}", dir_ind_reg_a, dir_ind_reg_b, reg),
            Xor(dir_ind_reg_a, dir_ind_reg_b, reg) => write!(f, "xor {}, {}, {}", dir_ind_reg_a, dir_ind_reg_b, reg),
            ZJump(dir) => write!(f, "zjmp {}", dir),
            LoadIndex(dir_ind_reg, dir_reg, reg) => write!(f, "ldi {}, {}, {}", dir_ind_reg, dir_reg, reg),
            StoreIndex(reg, dir_ind_reg, dir_reg) => write!(f, "sti {}, {}, {}", reg, dir_ind_reg, dir_reg),
            Fork(dir) => write!(f, "fork {}", dir),
            LongLoad(dir_ind, reg) => write!(f, "lld {}, {}", dir_ind, reg),
            LongLoadIndex(dir_ind_reg, dir_reg, reg) => write!(f, "lldi {}, {}, {}", dir_ind_reg, dir_reg, reg),
            LongFork(dir) => write!(f, "lfork {}", dir),
            Display(reg) => write!(f, "aff {}", reg),
        }
    }
}
