#![feature(try_from)]
#![feature(const_fn)]
#![feature(const_size_of)]

#[macro_use] extern crate log;
extern crate pest;
#[macro_use] extern crate pest_derive;
extern crate core;
extern crate machine;

mod var_instr;
mod property;
mod label;

use std::io::Write;
use std::mem;
use std::collections::HashMap;
use std::convert::TryFrom;
use pest::{Parser, Error};
use pest::inputs::{StringInput, Span};
use pest::iterators::Pair;
use machine::instruction::mem_size::MemSize;
use machine::instruction::Instruction;
use core::{Header, COREWAR_EXEC_MAGIC, PROG_NAME_LENGTH, COMMENT_LENGTH};
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

pub struct ParsedProgram {
    file_pair: AsmPair,
    properties: HashMap<String, (AsmSpan, Option<AsmSpan>)>,
    instructions: Vec<Instruction>,
}

pub fn parse_program(input: &str) -> Result<ParsedProgram, AsmError> {
    let mut pairs = AsmParser::parse_str(Rule::asm, input)?;

    let mut properties = HashMap::new();
    let mut label_offsets = HashMap::new();
    let mut var_instrs = Vec::new();
    let mut offset = 0;

    let file_pair = pairs.next().unwrap();
    for inner_pair in file_pair.clone().into_inner() {
        match inner_pair.as_rule() {
            Rule::props => for property_pair in inner_pair.into_inner() {
                let Property{ name, value } = Property::from(property_pair);
                properties.insert(name.as_str().to_string(), (name, value));
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

    let mut instructions = Vec::with_capacity(var_instrs.len());
    let mut offset = 0;
    for var_instr in &var_instrs {
        match var_instr.as_instr(offset, &label_offsets) {
            Ok(instr) => {
                offset += instr.mem_size();
                instructions.push(instr);
            },
            Err(LabelNotFound(label)) => return Err(Error::CustomErrorSpan {
                message: "label not found".into(),
                span: label.as_span().clone(),
            })
        }
    }

    Ok(ParsedProgram { file_pair, properties, instructions })
}

pub fn compile(parsed_program: &ParsedProgram) -> Result<Vec<u8>, AsmError> {
    let mut output = Vec::new();
    let &ParsedProgram { ref file_pair, ref properties, ref instructions } = parsed_program;

    let mut header = Header {
        magic: COREWAR_EXEC_MAGIC.to_be(),
        prog_name: [0u8; PROG_NAME_LENGTH + 1],
        prog_size: (instructions.iter().map(|x| x.mem_size()).sum::<usize>() as u32).to_be(),
        comment: [0u8; COMMENT_LENGTH + 1],
    };

    match properties.get("name") {
        Some(&(_, Some(ref value))) if value.as_str().is_empty() => return Err(Error::CustomErrorSpan {
            message: "name property's value can't be empty".into(),
            span: value.clone(),
        }),
        Some(&(_, Some(ref value))) => {
            let max_name_len = PROG_NAME_LENGTH;
            let value_len = value.as_str().as_bytes().len();
            let len = if max_name_len < value_len {
                eprintln!("name property's value as been clamped to {} chars.", max_name_len);
                max_name_len
            } else { value_len };

            let value_bytes = &value.as_str().as_bytes()[..len];
            (&mut header.prog_name[..]).write_all(value_bytes).unwrap();
        },
        Some(&(ref span, None)) => return Err(Error::CustomErrorPos {
            message: "name property need a value".into(),
            pos: span.start_pos(),
        }),
        None => return Err(Error::CustomErrorPos {
            message: "name property not found".into(),
            pos: file_pair.clone().into_span().start_pos(),
        }),
    }

    if let Some(&(_, Some(ref value))) = properties.get("comment") {
        let max_comment_len = COMMENT_LENGTH;
        let value_len = value.as_str().as_bytes().len();
        let len = if max_comment_len < value_len {
            eprintln!("comment property's value as been clamped to {} chars.", max_comment_len);
            max_comment_len
        } else { value_len };

        let value_bytes = &value.as_str().as_bytes()[..len];
        (&mut header.comment[..]).write_all(value_bytes).unwrap();
    }

    let header: [u8; mem::size_of::<Header>()] = unsafe { mem::transmute(header) };
    output.write_all(&header).unwrap();

    for instr in instructions {
        instr.write_to(&mut output).unwrap();
    }

    Ok(output)
}
