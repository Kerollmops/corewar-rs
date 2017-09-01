#![feature(try_from)]

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
use pest::inputs::StringInput;
use pest::iterators::Pair;
use machine::instruction::mem_size::MemSize;
use machine::instruction::Instruction;
use var_instr::VarInstr;
use property::Property;
use label::Label;

// force recompilation
const _GRAMMAR: &'static str = include_str!("asm.pest");

#[derive(Parser)]
#[grammar = "asm.pest"]
struct AsmParser;

type AsmPair = Pair<Rule, StringInput>;
pub type AsmError = Error<Rule, StringInput>;

#[derive(Debug)]
struct Champion {
    name: String,
    comment: String,
    program: Vec<Instruction>,
}

// fn retrieve_variable_instructions() -> (HashMap<Label, Offset>, Vec<VarInstr>) {
//     unimplemented!()
// }

pub fn compile<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<(), AsmError> {

    let mut content = String::new();
    input.read_to_string(&mut content).unwrap(); // FIXME: don't unwrap

    let pairs = AsmParser::parse_str(Rule::asm, &content)?;

    let mut properties = HashMap::new();
    let mut var_instrs = Vec::new();
    let mut offset = 0;
    let mut label_offsets = HashMap::new();

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
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
                    assert!(label_offsets.insert(label, offset).is_none(), "label already declared"); // FIXME: handle this error
                },
                _ => unreachable!(),
            };
        }
    }

    println!("properties:");
    for property in properties {
        println!("  prop: {:?}", property);
    }
    println!();

    println!("variable instructions");
    for var_instr in var_instrs {
        println!("  instr: {:?}", var_instr);
    }
    println!();

    println!("labels");
    for label in label_offsets {
        println!("  label: {:?}", label);
    }
    println!();

    Ok(())
}
