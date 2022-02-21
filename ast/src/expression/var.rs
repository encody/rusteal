use crate::{
    compilation_error::CompilationError,
    context::{CompilationBinding, CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::{prepend_stack, primitive::Primitive, Expression};

#[derive(Debug, Clone)]
pub enum Var {
    Temp(String),
    Global(String),
    Local(String),
}

impl Var {
    pub fn get_type<'a>(&self, context: &'a TypeContext) -> Result<&'a TypeEnum, TypeError> {
        let (identifier, scope) = match &self {
            Var::Temp(i) => (i, &context.temp_scope),
            Var::Global(i) => (i, &context.global_scope),
            Var::Local(i) => (i, &context.local_scope),
        };

        scope
            .get(&identifier)
            .ok_or(TypeError::UnboundIdentifier(self.clone()))
    }
}

pub struct Lvalue(pub Var);
pub struct Rvalue(pub Var);

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
        prepared_stack: Option<String>,
    ) -> Result<String, CompilationError> {
        match &self.0 {
            Var::Global(identifier) => Ok(format!(
                "{push_identifier}{OP_SEPARATOR}app_global_get",
                push_identifier =
                    Primitive::Byteslice(identifier.as_bytes().to_vec()).compile(context, None)?
            )),
            Var::Local(identifier) => Ok(format!(
                "{push_identifier}{OP_SEPARATOR}app_local_get", // app_local_get pops 2 elements (second is account identifier), which is why it is typed as a function instead of a simple primitive
                push_identifier =
                    Primitive::Byteslice(identifier.as_bytes().to_vec()).compile(context, None)?
            )),
            Var::Temp(identifier) => {
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
        .map(prepend_stack(prepared_stack))
    }
}
