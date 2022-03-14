use strum_macros::EnumString;

use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typesig,
    typing::{TypeEnum, TypeError, TypePrimitive, TypeVar},
};

use super::Expression;

// TODO: Incomplete
#[derive(Debug, Clone, PartialEq, EnumString)]
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
        Ok(match self {
            Txn::Sender | Txn::Receiver | Txn::CloseRemainderTo | Txn::Accounts => {
                typesig!(bytes)
            }
            _ => typesig!(int),
        })
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
