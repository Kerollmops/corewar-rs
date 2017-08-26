extern crate pest;
#[macro_use] extern crate pest_derive;
extern crate machine;

use std::io::{Read, Write};
use pest::{Parser, Error};
use pest::inputs::StringInput;

#[derive(Parser)]
#[grammar = "asm.pest"]
struct AsmParser;

pub type AsmError = Error<Rule, StringInput>;

pub fn compile<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<(), AsmError> {
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let pairs = AsmParser::parse_str(Rule::asm, &content)?;

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::prog_header => println!("Prog Header: {}", inner_pair.into_span().as_str()),
                Rule::instr => for inner_pair in inner_pair.into_inner() {
                    match inner_pair.as_rule() {
                        r => println!("{:?}: {}", r, inner_pair.into_span().as_str()),
                    }
                },
                Rule::label_decl => println!("Label: {}", inner_pair.into_span().as_str()),
                _ => unreachable!()
            };
        }
    }
    Ok(())
}
