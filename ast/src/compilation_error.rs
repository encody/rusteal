use crate::type_enum::TypeCheckError;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilationError {
    #[error("Type checking failed")]
    TypeCheck(#[from] TypeCheckError),
    #[error("Error parsing bytes string")]
    BytesStringParse(#[from] FromUtf8Error),
}
