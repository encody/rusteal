use std::ascii;

use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
};

use super::{Expression};

pub enum Primitive {
    UInt64(u64),
    Byteslice(Vec<u8>),
}

impl Expression for Primitive {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(TypeEnum::Simple(match self {
            Primitive::UInt64(_) => TypePrimitive::UInt64,
            Primitive::Byteslice(_) => TypePrimitive::Byteslice,
        }))
    }

    fn compile(
        &self,
        _: &CompilationContext,
        _: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        match self {
            Self::UInt64(value) => Ok(format!("int {value}")),
            Self::Byteslice(value) => {
                let escaped = String::from_utf8(
                    value
                        .into_iter()
                        .flat_map(|c| ascii::escape_default(*c))
                        .collect::<Vec<u8>>(),
                )?;

                Ok(format!("byte \"{escaped}\""))
            }
        }
    }
}
