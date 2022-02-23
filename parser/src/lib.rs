extern crate pest;
#[macro_use]
extern crate pest_derive;

use core::slice;
use std::{collections::HashMap, str::FromStr, vec};

use parse_error::ParseError;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use rusteal_ast::{
    contract::Contract,
    expression::{
        apply::Apply, binary::Binary, cond::Cond, primitive::Primitive, seq::Seq, txn::Txn,
        var::Var, Expression,
    },
    program::Program,
    struct_def::StructDef,
    typing::TypePrimitive,
    MAX_TEAL_VERSION,
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

fn parse_cond_arm(
    pair: Pair<Rule>,
) -> Result<(Box<dyn Expression>, Box<dyn Expression>), ParseError> {
    todo!()
}

fn fold_cond(
    mut i: vec::IntoIter<(Box<dyn Expression>, Box<dyn Expression>)>,
) -> Option<Box<Cond>> {
    let next = i.next();
    match next {
        Some((test, expr)) => Some(Box::new(Cond(test, expr, fold_cond(i)))),
        _ => None,
    }
}

fn parse_cond_expression(pair: Pair<Rule>) -> Result<Box<dyn Expression>, ParseError> {
    match pair.as_rule() {
        Rule::cond_expression => {
            let arms = pair
                .into_inner()
                .map(|p| parse_cond_arm(p))
                .collect::<Result<Vec<_>, ParseError>>()?;

            Ok(
                fold_cond(arms.into_iter()).ok_or(ParseError::EmptyCondExpression)?
                    as Box<dyn Expression>,
            )
        }
        _ => unreachable!(),
    }
}

fn parse_literal_expression(pair: Pair<Rule>) -> Result<Primitive, ParseError> {
    match pair.as_rule() {
        Rule::literal_expression => {
            let lit = pair.into_inner().next().unwrap();
            match lit.as_rule() {
                Rule::uint64 => Ok(Primitive::UInt64(lit.as_str().parse().unwrap())),
                Rule::boolean => Ok(Primitive::UInt64(if lit.as_str() == "false" {
                    0
                } else {
                    1
                })),
                Rule::bytes => Ok(Primitive::Byteslice(lit.as_str().as_bytes().to_vec())),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn parse_qualified_identifier(pair: Pair<Rule>) -> Result<Vec<&str>, ParseError> {
    match pair.as_rule() {
        Rule::qualified_identifier => pair
            .into_inner()
            .map(|i| parse_identifier(i))
            .collect::<Result<Vec<&str>, ParseError>>(),
        _ => unreachable!(),
    }
}

fn parse_binary_operation(
    lhs: Box<dyn Expression>,
    pair: Pair<Rule>,
) -> Result<Box<dyn Expression>, ParseError> {
    match pair.as_rule() {
        Rule::binary_operation => {
            let mut i = pair.into_inner();
            let operator = i.next().unwrap();
            let rhs = parse_expression(i.next().unwrap())?;
            match operator.as_str() {
                ";" => Ok(Box::new(Seq(lhs, Some(rhs)))),
                ">" => Ok(Box::new(Apply(
                    Box::new(Apply(Box::new(Binary::GreaterThan), rhs)),
                    lhs,
                ))),
                _ => todo!(),
            }
        }
        _ => unreachable!(),
    }
}

fn parse_expression(pair1: Pair<Rule>) -> Result<Box<dyn Expression>, ParseError> {
    match pair1.as_rule() {
        Rule::expression => {
            let mut i = pair1.into_inner();
            let pair = i.next().unwrap();

            let as_str = pair.as_str();
            let rule = pair.as_rule();
            let expr = match rule {
                // parenthesized/bracketed expressions result in nesting
                Rule::expression => parse_expression(i.next().unwrap()),
                Rule::literal_expression => parse_literal_expression(i.next().unwrap())
                    .map(|p| Box::new(p) as Box<dyn Expression>),
                Rule::cond_expression => parse_cond_expression(i.next().unwrap()),
                Rule::qualified_identifier => {
                    let i_vec = parse_qualified_identifier(i.next().unwrap())?;
                    match &i_vec[..] {
                        [i] => Ok(Box::new(Var(i.to_string())) as Box<dyn Expression>),
                        ["Txn", s] => Txn::from_str(s)
                            .map(|v| Box::new(v) as Box<dyn Expression>)
                            .map_err(|_| ParseError::UnknownQualifiedIdentifier(as_str)),
                        _ => Err(ParseError::UnknownQualifiedIdentifier(as_str)),
                    }
                }
                _ => unreachable!(),
            };

            let bin_pair = i.next();
            if let Some(bin_pair) = bin_pair {
                match bin_pair {
                    // Rule::binary_operation => {}
                    _ => todo!(),
                }
            }

            expr
        }
        _ => unreachable!(),
    }
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
