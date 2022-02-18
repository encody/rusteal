use crate::{
  compilation_error::CompilationError,
  type_enum::{TypeCheckError, TypeEnum, TypePrimitive},
};

use super::Expression;

#[derive(Debug)]
pub enum OnComplete {
  NoOp,
  OptIn,
  CloseOut,
  ClearState,
  UpdateApplication,
  DeleteApplication,
}

impl Expression for OnComplete {
  fn resolve(&self) -> Result<TypeEnum, TypeCheckError> {
    Ok(TypeEnum::Simple(TypePrimitive::UInt64))
  }

  fn compile(&self) -> Result<String, CompilationError> {
    Ok(format!("int {self:?}"))
  }
}

#[cfg(test)]
mod tests {
  use crate::expression::{constant::OnComplete, Expression};

  #[test]
  fn test() {
    let e = OnComplete::NoOp;
    assert_eq!(e.compile().unwrap(), "int NoOp");
  }
}
