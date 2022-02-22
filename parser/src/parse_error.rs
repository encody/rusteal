use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError<'a> {
    #[error("Invalid program name {0}")]
    InvalidProgramName(&'a str),
    #[error("Duplicate program name {0}")]
    DuplicateProgramName(&'a str),
    #[error("Invalid schema name {0}")]
    InvalidSchemaName(&'a str),
    #[error("Duplicate schema name {0}")]
    DuplicateSchemaName(&'a str),
    #[error("Cond expression must have at least one arm")]
    EmptyCondExpression,
    #[error("Unknown qualified identifier {0}")]
    UnknownQualifiedIdentifier(&'a str),
}
