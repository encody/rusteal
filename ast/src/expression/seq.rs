use crate::{
  compilation_error::CompilationError,
  type_enum::{TypeCheckError, TypeEnum, TypePrimitive},
  OP_SEPARATOR,
};

use super::Expression;

pub struct Seq(pub Vec<Box<dyn Expression>>);

impl Expression for Seq {
  fn resolve(&self) -> Result<TypeEnum, TypeCheckError> {
    self
      .0
      .last()
      .map_or(Ok(TypeEnum::Simple(TypePrimitive::Void)), |e| e.resolve())
  }

  fn compile(&self) -> Result<String, CompilationError> {
    let Self(expressions) = self;

    expressions
      .into_iter()
      .map(|e| e.compile())
      .collect::<Result<Vec<String>, CompilationError>>()
      .map(|s| s.join(OP_SEPARATOR))
  }
}
