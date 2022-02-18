pub const OP_SEPARATOR: &'static str = "\n";

pub mod compilation_error;
pub mod expression;
pub mod program;
pub mod type_enum;
pub mod label;

#[cfg(test)]
mod tests {
  use crate::expression::apply::Apply;
  use crate::expression::binary::Binary;
  use crate::expression::primitive::Primitive;
  use crate::expression::seq::Seq;
  use crate::program::Program;

  #[test]
  fn test_seq_int_bytes() {
    let compiled = Program {
      version: 5,
      body: Box::new(Seq(vec![
        Box::new(Primitive::UInt64(5)),
        Box::new(Primitive::Byteslice(b"test".to_vec())),
      ])),
    }
    .compile();
    println!("{}", compiled.unwrap());
  }

  #[test]
  fn test_types() {
    let program = Program {
      version: 5,
      body: Box::new(Seq(vec![
        Box::new(Apply(
          Box::new(Apply(
            Box::new(Binary::Equals),
            Box::new(Primitive::UInt64(5)),
          )),
          Box::new(Primitive::UInt64(5)),
        )),
        Box::new(Apply(
          Box::new(Apply(
            Box::new(Binary::GreaterThan),
            Box::new(Primitive::UInt64(5)),
          )),
          Box::new(Primitive::UInt64(6)),
        )),
        Box::new(Apply(
          Box::new(Apply(
            Box::new(Binary::NotEquals),
            Box::new(Primitive::Byteslice(b"test".to_vec())),
          )),
          Box::new(Primitive::Byteslice(b"testagain".to_vec())),
        )),
      ])),
    };
    println!("{:?}", program.type_check().unwrap());
    println!("{}", program.compile().unwrap());
  }
}
