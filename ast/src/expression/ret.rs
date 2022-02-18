use crate::{
  compilation_error::CompilationError,
  type_enum::{TypeCheckError, TypeEnum, TypePrimitive},
  OP_SEPARATOR,
};

use super::Expression;

pub enum Ret {
  Approve,
  Reject,
}

impl Expression for Ret {
  fn resolve(&self) -> Result<TypeEnum, TypeCheckError> {
    Ok(TypeEnum::Simple(TypePrimitive::Halt))
  }

  fn compile(&self) -> Result<String, CompilationError> {
    Ok(format!(
      "int {value}{OP_SEPARATOR}return",
      value = match self {
        Ret::Approve => "1",
        Ret::Reject => "0",
      }
    ))
  }
}

#[cfg(test)]
mod tests {
  use crate::expression::{ret::Ret, Expression};

  #[test]
  fn test() {
    let e = Ret::Approve;
    println!("{:?}", e.resolve());
    println!("{}", e.compile().unwrap());
  }
}
