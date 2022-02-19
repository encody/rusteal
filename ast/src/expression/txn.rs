use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    type_enum::{TypeEnum, TypeError, TypePrimitive},
};

use super::Expression;

// TODO: Incomplete
#[derive(Debug)]
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

    fn compile(&self, _: &CompilationContext) -> Result<String, CompilationError> {
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
