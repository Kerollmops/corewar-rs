extern crate pest;
#[macro_use] extern crate pest_derive;
extern crate core;
extern crate machine;

use std::io::{Read, Write};
use pest::{Parser, Error};
use pest::inputs::{StringInput, Span};
use machine::instruction::Instruction;
// use machine::instruction::parameter::{Direct, Indirect, Register};

// force recompilation
const _GRAMMAR: &'static str = include_str!("asm.pest");

#[derive(Parser)]
#[grammar = "asm.pest"]
struct AsmParser;

pub type AsmError = Error<Rule, StringInput>;

#[derive(Debug)]
struct Champion {
    name: String,
    comment: String,
    program: Vec<Instruction>,
}

// #[derive(Debug)]
// enum AsmParam<T> {
//     Complete(T),
//     Incomplete(T, ()),
// }

// #[derive(Debug)]
// enum AsmInstr {
//     Live(Direct),
//     Load(DirInd, Register),
//     Store(Register, IndReg),
//     Addition(Register, Register, Register),
//     Substraction(Register, Register, Register),
//     And(DirIndReg, DirIndReg, Register),
//     Or(DirIndReg, DirIndReg, Register),
//     Xor(DirIndReg, DirIndReg, Register),
//     ZJump(Direct),
//     LoadIndex(DirIndReg, DirReg, Register),
//     StoreIndex(Register, DirIndReg, DirReg),
//     Fork(Direct),
//     LongLoad(DirInd, Register),
//     LongLoadIndex(DirIndReg, DirReg, Register),
//     Longfork(Direct),
//     Display(Register),
// }

pub fn compile<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<(), AsmError> {

    let mut content = String::new();
    input.read_to_string(&mut content).unwrap(); // FIXME: don't unwrap

    let pairs = AsmParser::parse_str(Rule::asm, &content)?;

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::props => {

                    println!("props: ");
                    for inner_pair in inner_pair.into_inner() {
                        for inner_pair in inner_pair.into_inner() {
                            match inner_pair.as_rule() {
                                Rule::prop_name => println!("  name: {:?}", inner_pair.into_span().as_str()),
                                Rule::prop_value => for inner_pair in inner_pair.into_inner() {
                                    match inner_pair.as_rule() {
                                        Rule::quotted_string => for inner_pair in inner_pair.into_inner() {
                                            match inner_pair.as_rule() {
                                                Rule::inner_string => println!("  value: {:?}", inner_pair.into_span().as_str()),
                                                _ => unreachable!(),
                                            }
                                        },
                                        _ => unreachable!(),
                                    }
                                },
                                _ => unreachable!(),
                            }
                        }
                    }

                    println!();
                },
                Rule::instr => {
                    for inner_pair in inner_pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::instr_name_space => for inner_pair in inner_pair.into_inner() {
                                match inner_pair.as_rule() {
                                    Rule::instr_name => println!("  name: {:?}", inner_pair.into_span().as_str()),
                                    _ => unreachable!(),
                                }
                            },
                            Rule::parameter => for inner_pair in inner_pair.into_inner() {
                                match inner_pair.as_rule() {
                                    Rule::direct => println!("  direct: {:?}", inner_pair.into_span().as_str()),
                                    Rule::indirect => println!("  indirect: {:?}", inner_pair.into_span().as_str()),
                                    Rule::register => println!("  register: {:?}", inner_pair.into_span().as_str()),
                                    _ => unreachable!(),
                                }
                            },
                            _ => ()
                        }
                    }
                    println!();
                },
                Rule::label_decl => {
                    println!("label_decl:");
                    let mut tmp = inner_pair.into_inner();
                    let tmp = tmp.next().unwrap();
                    let tmp = tmp.into_span();
                    let label_name = tmp.as_str();
                    println!("  name: {}", label_name);
                    println!();
                },
                _ => (),
            };
        }
    }
    Ok(())
}
