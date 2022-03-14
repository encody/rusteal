use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typesig,
    typing::{TypeEnum, TypeError, TypePrimitive, TypeVar},
    OP_SEPARATOR,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Unary {
    Len,
    Not,
    ItoB,
    BtoI,
    BitInvert,
    Sqrt,
}

impl Expression for Unary {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(match self {
            Unary::Len | Unary::BtoI => typesig!(bytes -> int),
            Unary::Not | Unary::BitInvert | Unary::Sqrt => typesig!(int -> int),
            Unary::ItoB => typesig!(int -> bytes),
        })
    }

    fn compile(
        &self,
        _: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        let a = prepared_stack.pop().ok_or(CompilationError::MissingStack)?;
        let op = match self {
            Unary::Len => "len",
            Unary::Not => "!",
            Unary::BitInvert => "~",
            Unary::ItoB => "itob",
            Unary::BtoI => "btoi",
            Unary::Sqrt => "sqrt",
        };

        Ok(format!("{a}{OP_SEPARATOR}{op}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::apply;
    use crate::context::TypeContext;
    use crate::expression::apply::Apply;
    use crate::expression::primitive::Primitive;
    use crate::expression::Expr;
    use crate::expression::Expression;
    use crate::int;
    use crate::unop;

    use super::Unary;

    #[test]
    fn test_one() {
        let e =
        unop!(!
            unop!(len
                unop!(itob
                    unop!(sqrt
                        unop!(~
                            int!(5)
                        )
                    )
                )
            )
        );
        println!("{}", e.resolve(&TypeContext::default()).unwrap());
        println!("{}", e.compile_raw().unwrap());
    }

    #[test]
    #[should_panic(expected = "IrreconcilableTypes")]
    fn test_fail() {
        let e = unop!(btoi int!(5));
        println!("{}", e.resolve(&TypeContext::default()).unwrap());
        println!("{}", e.compile_raw().unwrap());
    }
}
