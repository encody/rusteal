use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    expression::{primitive::Primitive, Expression},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::Var;

pub struct Lvalue(pub Var);

impl Expression for Lvalue {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let type_enum = self.0.get_type(context)?;

        let assign = TypeEnum::Arrow(
            Box::new(type_enum.clone()),
            Box::new(TypeEnum::Simple(TypePrimitive::Void)),
        );

        Ok(match self.0 {
            Var::Local(..) => TypeEnum::Arrow(
                Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                Box::new(assign),
            ),
            _ => assign,
        })
    }

    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: Option<String>,
    ) -> Result<String, CompilationError> {
        match &self.0 {
            Var::Local(identifier) => Ok(vec![
                Primitive::Byteslice(identifier.as_bytes().to_vec()).compile(context, None)?,
                prepared_stack.ok_or(CompilationError::MissingStack)?,
                "app_local_put".to_string(),
            ]
            .join(OP_SEPARATOR)),
            _ => todo!(),
        }
    }
}
