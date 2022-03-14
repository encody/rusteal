use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typesig,
    typing::{TypeEnum, TypeError, TypePrimitive, TypeVar},
    OP_SEPARATOR,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Binary {
    // Comparison operators
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,

    // Mathematic operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exp,
    ShiftLeft,
    ShiftRight,

    // Bitwise operators
    BitOr,
    BitAnd,
    BitXor,

    // Logical operators
    And,
    Or,
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
            // function binary<T>(a: T, b: T): uint;
            // 'a -> 'a -> uint
            Binary::Equals | Binary::NotEquals => typesig!(:a -> :a -> int),
            // function binary(a: uint, b: uint): uint;
            // uint -> uint -> uint
            _ => typesig!(int -> int -> int),
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
            Binary::Plus => op(a, "+", b),
            Binary::Minus => op(a, "-", b),
            Binary::Multiply => op(a, "*", b),
            Binary::Divide => op(a, "/", b),
            Binary::Modulo => op(a, "%", b),
            Binary::Exp => op(a, "exp", b),
            Binary::ShiftLeft => op(a, "shl", b),
            Binary::ShiftRight => op(a, "shr", b),
            Binary::BitOr => op(a, "|", b),
            Binary::BitAnd => op(a, "&", b),
            Binary::BitXor => op(a, "^", b),
            Binary::And => op(a, "&&", b),
            Binary::Or => op(a, "||", b),
        }
    }
}
