use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError, TypePrimitive, TypeVar},
    OP_SEPARATOR,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Binary {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
}

fn op(l: String, s: &str, r: String) -> Result<String, CompilationError> {
    Ok(format!(
        "{l}{OP_SEPARATOR}{r}{OP_SEPARATOR}{}",
        s.to_string()
    ))
}

impl Expression for Binary {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(match self {
            // function binary(a: uint, b: uint): uint;
            // uint -> uint -> uint
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
            // function binary<T>(a: T, b: T): uint;
            // 'a -> 'a -> uint
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

    fn compile(
        &self,
        _: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        let b = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
        let a = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
        match self {
            Binary::Equals => op(a, "==", b),
            Binary::NotEquals => op(a, "!=", b),
            Binary::GreaterThan => op(a, ">", b),
            Binary::GreaterThanEquals => op(a, ">=", b),
            Binary::LessThan => op(a, "<", b),
            Binary::LessThanEquals => op(a, "<=", b),
        }
    }
}
