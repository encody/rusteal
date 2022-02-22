use std::ascii;

use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
};

use super::Expression;

#[derive(Debug)]
pub enum Primitive {
    UInt64(u64),
    Byteslice(Vec<u8>),
}

impl From<&str> for Primitive {
    fn from(s: &str) -> Self {
        Primitive::Byteslice(s.as_bytes().to_vec())
    }
}

// Cannot `impl<T: Borrow<String>> From<T>` because of `impl From<u64>` block below
impl From<String> for Primitive {
    fn from(s: String) -> Self {
        From::from(s.as_str())
    }
}

impl From<&String> for Primitive {
    fn from(s: &String) -> Self {
        From::from(s.as_str())
    }
}

impl From<u64> for Primitive {
    fn from(i: u64) -> Self {
        Primitive::UInt64(i)
    }
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
