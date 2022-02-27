use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    label::create_label_id,
    typing::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::{Expr, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct If(pub Expr, pub Expr);

impl Expression for If {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let If(true_expression, false_expression) = self;
        let mut true_type = true_expression.resolve(context)?;
        let mut false_type = false_expression.resolve(context)?;
        true_type.unify(&mut false_type)?;
        Ok(TypeEnum::Arrow(
            Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
            Box::new(true_type),
        ))
    }

    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        let If(true_expression, false_expression) = self;
        let true_compiled = true_expression.compile(context, &mut vec![])?;
        let false_compiled = false_expression.compile(context, &mut vec![])?;
        let else_label_id = format!("else{}", create_label_id());
        let endif_label_id = format!("endif{}", create_label_id());
        Ok(vec![
            prepared_stack.pop().ok_or(CompilationError::MissingStack)?,
            format!("bz {else_label_id}"),
            true_compiled,
            format!("b {endif_label_id}"),
            format!("{else_label_id}:"),
            false_compiled,
            format!("{endif_label_id}:"),
        ]
        .join(OP_SEPARATOR))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        apply, binop, bytes,
        context::TypeContext,
        expression::{apply::Apply, binary::Binary, primitive::Primitive, Expr, Expression},
        int,
    };

    use super::If;

    #[test]
    fn test() {
        let e = apply!(
            @fn Expr::If(Box::new(If(bytes!("true!".into()), bytes!("false!".into()))));
            @arg binop!((int!(4)) > (int!(2)));
        );
        println!("{:?}", e.resolve(&TypeContext::default()).unwrap());
        println!("{}", e.compile_raw().unwrap());
    }
}
