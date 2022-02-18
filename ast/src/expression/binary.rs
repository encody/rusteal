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

fn op(s: &str) -> Result<String, CompilationError> {
  Ok(s.to_string())
}

impl Expression for Binary {
  fn resolve(&self) -> Result<TypeEnum, TypeCheckError> {
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

  fn compile(&self) -> Result<String, CompilationError> {
    match self {
      Binary::Equals => op("=="),
      Binary::NotEquals => op("!="),
      Binary::GreaterThan => op(">"),
      Binary::GreaterThanEquals => op(">="),
      Binary::LessThan => op("<"),
      Binary::LessThanEquals => op("<="),
    }
  }
}
