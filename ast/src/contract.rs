use crate::{program::Program, struct_def::StructDef};

pub struct Contract<'a> {
    pub schema_global: StructDef<'a>,
    pub schema_local: StructDef<'a>,
    pub txn_approval: Program,
    pub txn_clear: Program,
}
