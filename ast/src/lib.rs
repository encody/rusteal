pub const MAX_TEAL_VERSION: u64 = 5;
pub const OP_SEPARATOR: &'static str = "\n";

pub mod compilation_error;
pub mod context;
pub mod contract;
pub mod expression;
pub mod label;
pub mod macros;
pub mod program;
pub mod struct_def;
pub mod typing;

#[cfg(test)]
mod tests {
    use crate::expression::apply::Apply;
    use crate::expression::binary::Binary;
    use crate::expression::bind::Bind;
    use crate::expression::cond::Cond;
    use crate::expression::constant::OnComplete;
    use crate::expression::primitive::Primitive;
    use crate::expression::ret::Ret;
    use crate::expression::seq::Seq;
    use crate::expression::txn::Txn;
    use crate::expression::Expr;
    use crate::program::Program;
    use crate::{apply, bind_let, binop, bytes, int, ret};

    #[test]
    fn test_seq_int_bytes() {
        let compiled = Program {
            version: 5,
            body: Expr::Seq(Box::new(Seq(
                Expr::Primitive(Primitive::UInt64(5)),
                Some(Expr::Primitive(Primitive::Byteslice(b"test".to_vec()))),
            ))),
        }
        .compile();
        println!("{}", compiled.unwrap());
    }

    #[test]
    fn let_binop_ret() {
        let program = Program {
            version: 5,
            body: bind_let!(
                int_eq = binop!((int!(5)) == (int!(5)));
                bind_let!(
                    int_gt = binop!((int!(6)) > (int!(5)));
                    bind_let!(
                        bytes_ne = binop!((bytes!("testagain".into())) != (bytes!("test".into())));
                        ret!(int!(1))
                    )
                )
            ),
        };
        println!("{:?}", program.type_check().unwrap());
        println!("{}", program.compile().unwrap());
    }

    #[test]
    fn main_conditional() {
        let program = Program {
            version: 5,
            body: Expr::Seq(Box::new(Seq(
                Expr::Cond(Box::new(Cond(
                    Expr::Apply(Box::new(Apply(
                        Expr::Apply(Box::new(Apply(
                            Expr::Binary(Binary::Equals),
                            Expr::Primitive(Primitive::UInt64(0)),
                        ))),
                        Expr::Txn(Txn::ApplicationID),
                    ))),
                    Expr::Primitive(Primitive::Byteslice(b"init".to_vec())),
                    Some(Box::new(Cond(
                        Expr::Apply(Box::new(Apply(
                            Expr::Apply(Box::new(Apply(
                                Expr::Binary(Binary::Equals),
                                Expr::OnComplete(OnComplete::NoOp),
                            ))),
                            Expr::Txn(Txn::OnCompletion),
                        ))),
                        Expr::Primitive(Primitive::Byteslice(b"noop".to_vec())),
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
