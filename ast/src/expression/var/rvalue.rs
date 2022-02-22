use crate::{
    compilation_error::CompilationError,
    context::{CompilationBinding, CompilationContext, TypeContext},
    expression::{primitive::Primitive, Expression},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::Var;

pub struct Rvalue(pub Var);

impl Expression for Rvalue {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let type_enum = self.0.get_type(context)?;

        if let Var::Local(..) = self.0 {
            Ok(TypeEnum::Arrow(
                Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                Box::new(type_enum.clone()),
            ))
        } else {
            Ok(type_enum.clone())
        }
    }

    fn compile(
        &self,
        context: &CompilationContext,
        _: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        match &self.0 {
            Var::Global(identifier) => Ok(format!(
                "{push_identifier}{OP_SEPARATOR}app_global_get",
                push_identifier = Primitive::Byteslice(identifier.as_bytes().to_vec())
                    .compile(context, &mut Vec::new())?
            )),
            Var::Local(identifier) => Ok(format!(
                "{push_identifier}{OP_SEPARATOR}app_local_get", // app_local_get pops 2 elements (second is account identifier), which is why it is typed as a function instead of a simple primitive
                push_identifier = Primitive::Byteslice(identifier.as_bytes().to_vec())
                    .compile(context, &mut Vec::new())?
            )),
            Var::Bind(identifier) => {
                let binding = context.scope.get(&identifier).ok_or::<CompilationError>(
                    // should never happen if type checking is run before compilation
                    TypeError::UnboundIdentifier(self.0.clone()).into(),
                )?;

                Ok(match binding {
                    CompilationBinding::Replacement(s) => s.to_string(),
                    CompilationBinding::ScratchVar(i) => format!("load {i}"),
                })
            }
        }
    }
}
