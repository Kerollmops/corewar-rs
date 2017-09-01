#![feature(try_from)]

#[macro_use] extern crate log;
extern crate pest;
#[macro_use] extern crate pest_derive;
extern crate core;
extern crate machine;

mod var_instr;
mod property;
mod label;

use std::io::{Read, Write};
use std::collections::HashMap;
use std::convert::TryFrom;
use pest::{Parser, Error};
use pest::inputs::{StringInput, Span};
use pest::iterators::Pair;
use machine::instruction::mem_size::MemSize;
use machine::instruction::Instruction;
use var_instr::variable::LabelNotFound;
use var_instr::VarInstr;
use property::Property;
use label::Label;

// force recompilation
const _GRAMMAR: &'static str = include_str!("asm.pest");

#[derive(Parser)]
#[grammar = "asm.pest"]
struct AsmParser;

type AsmPair = Pair<Rule, StringInput>;
type AsmSpan = Span<StringInput>;
pub type AsmError = Error<Rule, StringInput>;

#[derive(Debug)]
struct Champion {
    name: String,
    comment: String,
    program: Vec<Instruction>,
}

pub fn compile<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<(), AsmError> {

    let mut content = String::new();
    input.read_to_string(&mut content).unwrap(); // FIXME: don't unwrap

    let mut pairs = AsmParser::parse_str(Rule::asm, &content)?;

    let mut properties = HashMap::new();
    let mut label_offsets = HashMap::new();
    let mut var_instrs = Vec::new();
    let mut offset = 0;

    for inner_pair in pairs.next().unwrap().into_inner() {
        match inner_pair.as_rule() {
            Rule::props => for property_pair in inner_pair.into_inner() {
                let Property{ name, value } = Property::from(property_pair);
                properties.insert(name, value);
            },
            Rule::instr => {
                let var_instr = VarInstr::try_from(inner_pair)?;
                offset += var_instr.mem_size();
                var_instrs.push(var_instr);
            },
            Rule::label_decl => {
                let label = Label::from(inner_pair);
                if label_offsets.insert(label.clone(), offset).is_some() {
                    return Err(Error::CustomErrorSpan {
                        message: "label already declared".into(),
                        span: label.as_span().clone(),
                    })
                }
            },
            _ => (),
        };
    }

    let mut instrs = Vec::with_capacity(var_instrs.len());
    let mut offset = 0;
    for var_instr in &var_instrs {
        match var_instr.as_instr(offset, &label_offsets) {
            Ok(instr) => {
                offset += instr.mem_size();
                instrs.push(instr);
            },
            Err(LabelNotFound(label)) => return Err(Error::CustomErrorSpan {
                message: "label not found".into(),
                span: label.as_span().clone(),
            })
        }
    }

    Ok(())
}
