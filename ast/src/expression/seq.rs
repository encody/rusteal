use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::{Expr, Expression};

pub struct Seq(pub Box<Expr>, pub Option<Box<Expr>>);

impl Expression for Seq {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        match (self.0.resolve(context)?, &self.1) {
            (h @ TypeEnum::Simple(TypePrimitive::Halt), _) | (h, None) => Ok(h),
            (TypeEnum::Simple(TypePrimitive::Void), Some(t)) => t.resolve(context),
            _ => todo!(),
        }
    }

    fn compile(
        &self,
        context: &CompilationContext,
        _: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        let Self(head, tail) = self;

        Ok(match tail {
            Some(tail) => format!(
                "{head}{OP_SEPARATOR}{tail}",
                head = head.compile(context, &mut vec![])?,
                tail = tail.compile(context, &mut vec![])?,
            ),
            None => head.compile(context, &mut vec![])?,
        })
    }
}
