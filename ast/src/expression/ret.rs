use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typesig,
    typing::{TypeEnum, TypeError, TypePrimitive, TypeVar},
    OP_SEPARATOR,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Ret;

impl Expression for Ret {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(typesig!(int -> halt))
    }

    fn compile(
        &self,
        _context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        Ok(format!(
            "{}{OP_SEPARATOR}return",
            prepared_stack.pop().ok_or(CompilationError::MissingStack)?
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::TypeContext,
        expression::{apply::Apply, primitive::Primitive, ret::Ret, Expr, Expression},
    };

    #[test]
    fn test() {
        let e = Expr::Apply(Box::new(Apply(
            Expr::Ret(Ret),
            Expr::Primitive(Primitive::UInt64(1)),
        )));
        println!("{:?}", e.resolve(&TypeContext::default()));
        println!("{}", e.compile_raw().unwrap());
    }
}
