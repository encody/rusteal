pub const MAX_TEAL_VERSION: u64 = 5;
pub const OP_SEPARATOR: &'static str = "\n";

pub mod compilation_error;
pub mod context;
pub mod contract;
pub mod expression;
pub mod label;
pub mod program;
pub mod struct_def;
pub mod typing;

#[cfg(test)]
mod tests {
    use crate::expression::apply::Apply;
    use crate::expression::binary::Binary;
    use crate::expression::cond::Cond;
    use crate::expression::constant::OnComplete;
    use crate::expression::primitive::Primitive;
    use crate::expression::seq::Seq;
    use crate::expression::txn::Txn;
    use crate::expression::Expr;
    use crate::program::Program;

    #[test]
    fn test_seq_int_bytes() {
        let compiled = Program {
            version: 5,
            body: Box::new(Expr::Seq(Seq(
                Box::new(Expr::Primitive(Primitive::UInt64(5))),
                Some(Box::new(Expr::Primitive(Primitive::Byteslice(
                    b"test".to_vec(),
                )))),
            ))),
        }
        .compile();
        println!("{}", compiled.unwrap());
    }

    #[test]
    fn test_types() {
        let program = Program {
            version: 5,
            body: Box::new(Expr::Seq(Seq(
                Box::new(Expr::Apply(Apply(
                    Box::new(Expr::Apply(Apply(
                        Box::new(Expr::Binary(Binary::Equals)),
                        Box::new(Expr::Primitive(Primitive::UInt64(5))),
                    ))),
                    Box::new(Expr::Primitive(Primitive::UInt64(5))),
                ))),
                Some(Box::new(Expr::Seq(Seq(
                    Box::new(Expr::Apply(Apply(
                        Box::new(Expr::Apply(Apply(
                            Box::new(Expr::Binary(Binary::GreaterThan)),
                            Box::new(Expr::Primitive(Primitive::UInt64(5))),
                        ))),
                        Box::new(Expr::Primitive(Primitive::UInt64(6))),
                    ))),
                    Some(Box::new(Expr::Apply(Apply(
                        Box::new(Expr::Apply(Apply(
                            Box::new(Expr::Binary(Binary::NotEquals)),
                            Box::new(Expr::Primitive(Primitive::Byteslice(b"test".to_vec()))),
                        ))),
                        Box::new(Expr::Primitive(Primitive::Byteslice(b"testagain".to_vec()))),
                    )))),
                )))),
            ))),
        };
        println!("{:?}", program.type_check().unwrap());
        println!("{}", program.compile().unwrap());
    }

    #[test]
    fn main_conditional() {
        let program = Program {
            version: 5,
            body: Box::new(Expr::Seq(Seq(
                Box::new(Expr::Cond(Cond(
                    Box::new(Expr::Apply(Apply(
                        Box::new(Expr::Apply(Apply(
                            Box::new(Expr::Binary(Binary::Equals)),
                            Box::new(Expr::Primitive(Primitive::UInt64(0))),
                        ))),
                        Box::new(Expr::Txn(Txn::ApplicationID)),
                    ))),
                    Box::new(Expr::Primitive(Primitive::Byteslice(b"init".to_vec()))),
                    Some(Box::new(Cond(
                        Box::new(Expr::Apply(Apply(
                            Box::new(Expr::Apply(Apply(
                                Box::new(Expr::Binary(Binary::Equals)),
                                Box::new(Expr::OnComplete(OnComplete::NoOp)),
                            ))),
                            Box::new(Expr::Txn(Txn::OnCompletion)),
                        ))),
                        Box::new(Expr::Primitive(Primitive::Byteslice(b"noop".to_vec()))),
                        None,
                    ))),
                ))),
                None,
            ))),
        };
        println!("{:?}", program.type_check().unwrap());
        println!("{}", program.compile().unwrap());
    }
}
