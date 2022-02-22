use strum_macros::EnumString;

use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError, TypePrimitive},
};

use super::Expression;

// TODO: Incomplete
#[derive(Debug, EnumString)]
pub enum Txn {
    Sender,
    Fee,
    Receiver,
    Amount,
    CloseRemainderTo,
    GroupIndex,
    ApplicationID,
    OnCompletion,
    Accounts,
    NumAccounts,
}

impl Expression for Txn {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(TypeEnum::Simple(match self {
            Txn::Sender | Txn::Receiver | Txn::CloseRemainderTo | Txn::Accounts => {
                TypePrimitive::Byteslice
            }
            _ => TypePrimitive::UInt64,
        }))
    }

    fn compile(
        &self,
        _: &CompilationContext,
        _: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        Ok(format!("txn {:?}", self))
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::txn::Txn;

    #[test]
    fn test() {
        println!("{:?}", Txn::Sender);
    }
}
