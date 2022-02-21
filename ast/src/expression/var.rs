use crate::{
    compilation_error::CompilationError,
    context::{CompilationBinding, CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::{primitive::Primitive, Expression};

pub enum Var {
    Temp(String),
    Global(String),
    Local(String),
}

pub struct Rvalue(pub Var);
pub struct Lvalue(pub Var);

impl Expression for Rvalue {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        let (identifier, scope) = match &self.0 {
            Var::Temp(i) => (i, &context.temp_scope),
            Var::Global(i) => (i, &context.global_scope),
            Var::Local(i) => (i, &context.local_scope),
        };

        let type_enum = scope
            .get(&identifier)
            .ok_or(TypeError::UnboundIdentifier(identifier.to_string()))
            .map(|t| t.to_owned())?;

        if let Var::Local(..) = self.0 {
            Ok(TypeEnum::Arrow(
                Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                Box::new(type_enum),
            ))
        } else {
            Ok(type_enum)
        }
    }

    fn compile(&self, context: &CompilationContext) -> Result<String, CompilationError> {
        match &self.0 {
            Var::Global(identifier) => Ok(format!(
                "{push_identifier}{OP_SEPARATOR}app_global_get",
                push_identifier =
                    Primitive::Byteslice(identifier.as_bytes().to_vec()).compile(context)?
            )),
            Var::Local(identifier) => Ok(format!(
                "{push_identifier}{OP_SEPARATOR}app_local_get", // app_local_get pops 2 elements (second is account identifier), which is why it is typed as a function instead of a simple primitive
                push_identifier =
                    Primitive::Byteslice(identifier.as_bytes().to_vec()).compile(context)?
            )),
            Var::Temp(identifier) => {
                let binding = context.scope.get(&identifier).ok_or::<CompilationError>(
                    // should never happen if type checking is run before compilation
                    TypeError::UnboundIdentifier(identifier.to_string()).into(),
                )?;

                Ok(match binding {
                    CompilationBinding::Replacement(s) => s.to_string(),
                    CompilationBinding::ScratchVar(i) => format!("load {i}"),
                })
            }
        }
    }
}
