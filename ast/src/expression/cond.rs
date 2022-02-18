use crate::{
  compilation_error::CompilationError,
  label::create_label_id,
  type_enum::{TypeCheckError, TypeEnum, TypePrimitive},
  OP_SEPARATOR,
};

use super::Expression;

pub struct Cond(
  pub Box<dyn Expression>,
  pub Box<dyn Expression>,
  pub Option<Box<Self>>,
);

impl Expression for Cond {
  fn resolve(&self) -> Result<TypeEnum, TypeCheckError> {
    let Cond(test, body, continuation) = self;

    test
      .resolve()?
      .unify(&mut TypeEnum::Simple(TypePrimitive::UInt64))?;
    let mut body_type = body.resolve()?;
    if let Some(ref c) = continuation {
      let mut continuation_type = c.resolve()?;
      body_type.unify(&mut continuation_type)?;
    }

    Ok(body_type)
  }

  fn compile(&self) -> Result<String, CompilationError> {
    let label_id = format!("cond{}", create_label_id());
    let Cond(test, body, continuation) = self;

    let continuation = if let Some(c) = continuation {
      c.compile()?
    } else {
      "err".to_string()
    };

    let pieces = vec![
      test.compile()?,
      format!("bnz {label_id}"),
      continuation,
      format!("{label_id}:"),
      body.compile()?,
    ];

    Ok(pieces.join(OP_SEPARATOR))
  }
}

#[cfg(test)]
mod tests {
  use crate::expression::{cond::Cond, primitive::Primitive, Expression};

  #[test]
  fn test() {
    let prog = Cond(
      Box::new(Primitive::UInt64(0)),
      Box::new(Primitive::Byteslice(b"hello".to_vec())),
      Some(Box::new(Cond(
        Box::new(Primitive::UInt64(1)),
        Box::new(Primitive::UInt64(6)),
        None,
      ))),
    );
    println!("{:?}", prog.resolve());
    println!("{}", prog.compile().unwrap());
  }
}
