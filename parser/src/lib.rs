extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

use parse_error::ParseError;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use rusteal_ast::{
    contract::Contract, expression::{Expression, cond::Cond}, program::Program, struct_def::StructDef,
    type_enum::TypePrimitive, MAX_TEAL_VERSION,
};
use thiserror::Error;

mod parse_error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RustealParser;

fn parse_identifier(pair: Pair<Rule>) -> Result<&str, ParseError> {
    match pair.as_rule() {
        Rule::identifier => Ok(pair.as_str()),
        _ => unreachable!(),
    }
}

fn parse_function_def(pair: Pair<Rule>) -> Result<Box<dyn Expression>, ParseError> {
    todo!()
}

fn parse_cond_expression(pair: Pair<Rule>) -> Result<Cond, ParseError> {
    todo!()
}

fn parse_expression(pair: Pair<Rule>) -> Result<Box<dyn Expression>, ParseError> {
    todo!()
}

fn parse_prog(pair: Pair<Rule>) -> Result<(&str, Program), ParseError> {
    match pair.as_rule() {
        Rule::prog => {
            let mut i = pair.into_inner();
            let identifier = parse_identifier(i.next().unwrap())?;
            let expression = parse_expression(i.next().unwrap())?;
            Ok((
                identifier,
                Program {
                    version: MAX_TEAL_VERSION,
                    body: expression,
                },
            ))
        }
        _ => unreachable!(),
    }
}

fn parse_datatype(pair: Pair<Rule>) -> Result<TypePrimitive, ParseError> {
    match (pair.as_rule(), pair.as_str()) {
        (Rule::datatype, "uint64") => Ok(TypePrimitive::UInt64),
        (Rule::datatype, "bytes") => Ok(TypePrimitive::Byteslice),
        _ => unreachable!(),
    }
}

fn parse_typed_field(pair: Pair<Rule>) -> Result<(&str, TypePrimitive), ParseError> {
    match pair.as_rule() {
        Rule::typed_field => {
            let mut i = pair.into_inner();
            let identifier = parse_identifier(i.next().unwrap())?;
            let datatype = parse_datatype(i.next().unwrap())?;
            Ok((identifier, datatype))
        }
        _ => unreachable!(),
    }
}

fn parse_struct_def(pair: Pair<Rule>) -> Result<StructDef, ParseError> {
    match pair.as_rule() {
        Rule::struct_def => Ok(StructDef {
            fields: pair
                .into_inner()
                .map(|p| parse_typed_field(p))
                .collect::<Result<HashMap<&str, TypePrimitive>, ParseError>>()?,
        }),
        _ => unreachable!(),
    }
}

fn parse_schema(pair: Pair<Rule>) -> Result<(&str, StructDef), ParseError> {
    match pair.as_rule() {
        Rule::schema => {
            let mut i = pair.into_inner();
            let name = parse_identifier(i.next().unwrap())?;
            let struct_def = parse_struct_def(i.next().unwrap())?;
            Ok((name, struct_def))
        }
        _ => unreachable!(),
    }
}

fn parse_contract(pairs: Pairs<Rule>) -> Result<Contract, ParseError> {
    let mut txn_approval: Option<Program> = None;
    let mut txn_clear: Option<Program> = None;
    let mut schema_global: Option<StructDef> = None;
    let mut schema_local: Option<StructDef> = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::prog => {
                let (name, prog) = parse_prog(pair)?;
                let o = match name {
                    "approval" => &mut txn_approval,
                    "clear" => &mut txn_clear,
                    _ => return Err(ParseError::InvalidProgramName(name)),
                };
                match o {
                    Some(_) => return Err(ParseError::DuplicateProgramName(name)),
                    None => *o = Some(prog),
                }
            }
            Rule::schema => {
                let (name, schema) = parse_schema(pair)?;
                let o = match name {
                    "global" => &mut schema_global,
                    "local" => &mut schema_local,
                    _ => return Err(ParseError::InvalidSchemaName(name)),
                };
                match o {
                    Some(_) => return Err(ParseError::DuplicateSchemaName(name)),
                    None => *o = Some(schema),
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(Contract {
        txn_approval: txn_approval.unwrap_or_else(|| Program::default()),
        txn_clear: txn_clear.unwrap_or_else(|| Program::default()),
        schema_global: schema_global.unwrap_or_else(|| StructDef::default()),
        schema_local: schema_local.unwrap_or_else(|| StructDef::default()),
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use pest::Parser;

    use crate::Rule;
    use crate::RustealParser;

    #[test]
    fn test() {
        let unparsed_file = fs::read_to_string("examples/1.rteal").expect("could not open file");
        let file = RustealParser::parse(Rule::contract, &unparsed_file)
            .expect("successful parse")
            .next()
            .unwrap();

        let pairs = file.into_inner();

        // for pair in pairs {
        //     println!("{:?}", pair);
        //     match pair.as_rule() {

        //     }
        // }

        // println!("{:?}", pairs);
    }
}
