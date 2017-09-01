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
use pest::{Parser, Error};
use pest::inputs::StringInput;
use pest::iterators::Pair;
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
type Offset = usize;

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

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::props => {
                    println!("props: ");
                    for property_pair in inner_pair.into_inner() {
                        let Property{ name, value } = Property::from(property_pair);
                        println!("  .{} {:?}", name, value);
                        properties.insert(name, value);
                    }
                    println!();
                },
                Rule::instr => {

                    println!();
                },
                Rule::label_decl => {
                    println!("label_decl:");
                    let Label{ name } = Label::from(inner_pair);
                    println!("  name: {}", name);
                    println!();
                },
                _ => (),
            };
        }
    }
    Ok(())
}
