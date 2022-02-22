use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, is_a},
    character::{
        complete::char,
        complete::{none_of, one_of},
    },
    combinator::{map_res, recognize, map_opt, self, map},
    multi::{many0, many1},
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};
use parse_error::ParseError;
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

fn uint64(input: &str) -> IResult<&str, Box<dyn Expression>> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |r: &str| {
            r.replace("_", "")
                .parse()
                .map(|v| Box::new(Primitive::UInt64(v)) as Box<dyn Expression>)
        },
    )(input)
}

fn byteslice_quoted(input: &str) -> IResult<&str, Box<dyn Expression>> {
    map(delimited(char('"'), quoted_inner, char('"')), |r| {
        Box::new(Primitive::from(r)) as Box<dyn Expression>
    })(input)
}

fn quoted_inner(input: &str) -> IResult<&str, &str> {
    recognize(escaped(is_not("\\\""), '\\', is_a("\\\"")))(input)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{byteslice_quoted, uint64};

    #[test]
    fn test_parser() {
        println!("{}", uint64("1_34_4.3838test").unwrap().1.compile_raw().unwrap());
        println!(
            "{}",
            byteslice_quoted(r#""inner\\<backslash,\"<double quote"a"#).unwrap().1.compile_raw().unwrap()
        );
    }

    #[test]
    fn test() {
        let unparsed_file = fs::read_to_string("examples/1.rteal").expect("could not open file");
        // let file = RustealParser::parse(Rule::contract, &unparsed_file)
        //     .expect("successful parse")
        //     .next()
        //     .unwrap();

        // let pairs = file.into_inner();

        // for pair in pairs {
        //     println!("{:?}", pair);
        //     match pair.as_rule() {

        //     }
        // }

        // println!("{:?}", pairs);
    }
}
