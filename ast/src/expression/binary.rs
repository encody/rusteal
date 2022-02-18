use crate::{
    compilation_error::CompilationError,
    type_enum::{TypeCheckError, TypeEnum, TypePrimitive, TypeVar},
};

use super::Expression;

pub enum Binary {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
}

// fn format_binary_op(a: &str, op: &str, b: &str) -> String {
//   format!("{a}{OP_SEPARATOR}{b}{OP_SEPARATOR}{op}")
// }

fn op(s: &str) -> Result<String, CompilationError> {
    Ok(s.to_string())
}

impl Expression for Binary {
    fn resolve(&self) -> Result<TypeEnum, TypeCheckError> {
        Ok(match self {
            Binary::GreaterThan
            | Binary::GreaterThanEquals
            | Binary::LessThan
            | Binary::LessThanEquals => TypeEnum::Arrow(
                Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                Box::new(TypeEnum::Arrow(
                    Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                    Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                )),
            ),
            Binary::Equals | Binary::NotEquals => {
                let tv = TypeVar::new();
                TypeEnum::Arrow(
                    Box::new(TypeEnum::Var(tv.clone())),
                    Box::new(TypeEnum::Arrow(
                        Box::new(TypeEnum::Var(tv.clone())),
                        Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                    )),
                )
            }
        })
    }

    fn compile(&self) -> Result<String, CompilationError> {
        match self {
            // Binary::Equals(ref a, ref b) => Ok(format_binary_op(&a.compile()?, "==", &b.compile()?)),
            Binary::Equals => op("=="),
            Binary::NotEquals => op("!="),
            Binary::GreaterThan => op(">"),
            Binary::GreaterThanEquals => op(">="),
            Binary::LessThan => op("<"),
            Binary::LessThanEquals => op("<="),
        }
    }
}
