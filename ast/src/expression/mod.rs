use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError},
};

pub mod apply;
pub mod binary;
pub mod bind;
pub mod cond;
pub mod constant;
pub mod primitive;
pub mod ret;
pub mod seq;
pub mod txn;
pub mod var;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Apply(Box<apply::Apply>),
    Binary(binary::Binary),
    Bind(Box<bind::Bind>),
    Cond(Box<cond::Cond>),
    OnComplete(constant::OnComplete),
    Primitive(primitive::Primitive),
    Ret(ret::Ret),
    Seq(Box<seq::Seq>),
    Txn(txn::Txn),
    LVal(var::LVal),
    RVal(var::RVal),
}

pub trait Expression {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError>;
    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError>;

    fn compile_raw(&self) -> Result<String, CompilationError> {
        self.compile(&CompilationContext::default(), &mut Vec::new())
    }
}

impl Expression for Expr {
    fn resolve(&self, context: &TypeContext) -> Result<TypeEnum, TypeError> {
        match self {
            Expr::Apply(expr) => expr.resolve(context),
            Expr::Binary(expr) => expr.resolve(context),
            Expr::Bind(expr) => expr.resolve(context),
            Expr::Cond(expr) => expr.resolve(context),
            Expr::OnComplete(expr) => expr.resolve(context),
            Expr::Primitive(expr) => expr.resolve(context),
            Expr::Ret(expr) => expr.resolve(context),
            Expr::Seq(expr) => expr.resolve(context),
            Expr::Txn(expr) => expr.resolve(context),
            Expr::LVal(expr) => expr.resolve(context),
            Expr::RVal(expr) => expr.resolve(context),
        }
    }

    fn compile(
        &self,
        context: &CompilationContext,
        prepared_stack: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        match self {
            Expr::Apply(expr) => expr.compile(context, prepared_stack),
            Expr::Binary(expr) => expr.compile(context, prepared_stack),
            Expr::Bind(expr) => expr.compile(context, prepared_stack),
            Expr::Cond(expr) => expr.compile(context, prepared_stack),
            Expr::OnComplete(expr) => expr.compile(context, prepared_stack),
            Expr::Primitive(expr) => expr.compile(context, prepared_stack),
            Expr::Ret(expr) => expr.compile(context, prepared_stack),
            Expr::Seq(expr) => expr.compile(context, prepared_stack),
            Expr::Txn(expr) => expr.compile(context, prepared_stack),
            Expr::LVal(expr) => expr.compile(context, prepared_stack),
            Expr::RVal(expr) => expr.compile(context, prepared_stack),
        }
    }
}
