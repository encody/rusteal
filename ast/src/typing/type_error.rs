use thiserror::Error;

use crate::expression::var::Var;

use super::{type_enum::TypeEnum, type_primitive::TypePrimitive, type_var::TypeVar};

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Mismatched types: {0:?} and {1:?}")]
    MismatchedTypes(TypePrimitive, TypePrimitive),
    #[error("Irreconcilable types: {0:?} and {1:?}")]
    IrreconcilableTypes(TypeEnum, TypeEnum),
    #[error("Unresolvable type variable in expression: {0:?} in {1:?}")]
    UnresolvableTypeVariable(TypeVar, TypeEnum),
    #[error("Stack underflow: {0:?}")]
    StackUnderflow(TypeEnum),
    #[error("Attempt to call a non-function expression: {0:?}")]
    NonFunctionApplication(TypeEnum),
    #[error("Unbound identifier: {0:?}")]
    UnboundIdentifier(Var),
}
