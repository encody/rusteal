use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use thiserror::Error;

use crate::expression::var::Var;

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

#[derive(Debug, PartialEq, Clone)]
pub enum TypeEnum {
    Simple(TypePrimitive),
    Arrow(Box<TypeEnum>, Box<TypeEnum>),
    Var(TypeVar),
}

impl TypeEnum {
    fn used_tvars(&self) -> Vec<usize> {
        match self {
            TypeEnum::Var(v) => {
                vec![v.id]
            }
            TypeEnum::Arrow(a, b) => {
                // must maintain element order AND uniqueness
                let used = a.used_tvars();
                used.clone()
                    .into_iter()
                    .chain(b.used_tvars().into_iter().filter(|e| !used.contains(e)))
                    .collect::<Vec<usize>>()
            }
            _ => {
                vec![]
            }
        }
    }
}

impl Display for TypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tvars = self.used_tvars();
        write!(f, "{}", self.stringify_with_tvars(&tvars))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypePrimitive {
    Void,
    UInt64,
    Byteslice,
    Halt,
}

impl Display for TypePrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TypePrimitive::Void => "<void>",
                TypePrimitive::UInt64 => "int",
                TypePrimitive::Byteslice => "bytes",
                TypePrimitive::Halt => "<halt>",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct TypeVar {
    id: usize,
    pub value: Rc<RefCell<Box<Option<TypeEnum>>>>,
}

impl PartialEq for TypeVar {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

static TYPE_VAR_ID: AtomicUsize = AtomicUsize::new(0);

impl TypeEnum {
    pub fn unify(&mut self, other: &mut Self) -> Result<(), TypeError> {
        match (&mut self.clone(), &mut other.clone()) {
            (a, b) if a == b => Ok(()),
            (TypeEnum::Var(tv), _) => match ((*tv.value).borrow_mut()).as_mut() {
                Some(ref mut x) => x.unify(other),
                None if other.contains(tv) => Err(TypeError::UnresolvableTypeVariable(
                    tv.clone(),
                    other.clone(),
                )),
                inner => {
                    *inner = Some(other.clone());
                    Ok(())
                }
            },
            (_, TypeEnum::Var(_)) => other.unify(self),
            (TypeEnum::Simple(TypePrimitive::Halt), TypeEnum::Simple(_))
            | (TypeEnum::Simple(_), TypeEnum::Simple(TypePrimitive::Halt)) => Ok(()),
            (TypeEnum::Simple(a), TypeEnum::Simple(b)) if a == b => Ok(()),
            (TypeEnum::Arrow(ref mut a1, ref mut a2), TypeEnum::Arrow(ref mut b1, ref mut b2)) => {
                a1.unify(b1).and(a2.unify(b2))
            }
            (a, b) => Err(TypeError::IrreconcilableTypes(a.clone(), b.clone())),
        }
    }

    pub fn contains(&self, other: &TypeVar) -> bool {
        match self {
            TypeEnum::Var(v) => {
                v == other
                    || if let Some(ref value) = **v.value.borrow() {
                        value.contains(other)
                    } else {
                        false
                    }
            }
            TypeEnum::Arrow(a, b) => a.contains(other) || b.contains(other),
            _ => false,
        }
    }

    fn stringify_with_tvars(&self, tvars: &Vec<usize>) -> String {
        format!(
            "{}",
            match self {
                TypeEnum::Simple(s) => format!("{}", s),
                TypeEnum::Arrow(a, b) => format!(
                    "{} -> {}",
                    a.stringify_with_tvars(tvars),
                    b.stringify_with_tvars(tvars)
                ),
                TypeEnum::Var(v) => format!(
                    "'{}",
                    (tvars.iter().position(|x| x == &v.id).unwrap() as u8 + ('a' as u8)) as char
                ),
            }
        )
    }
}

impl TypeVar {
    pub fn new() -> Self {
        Self {
            id: TYPE_VAR_ID.fetch_add(1, Ordering::SeqCst),
            value: Rc::new(RefCell::new(Box::new(None))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TypeEnum, TypePrimitive, TypeVar};

    #[test]
    fn test_identical_simple() {
        let mut a = TypeEnum::Simple(TypePrimitive::UInt64);
        let mut b = TypeEnum::Simple(TypePrimitive::UInt64);
        a.unify(&mut b).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    #[should_panic(expected = "IrreconcilableTypes")]
    fn test_different_simple() {
        let mut a = TypeEnum::Simple(TypePrimitive::UInt64);
        let mut b = TypeEnum::Simple(TypePrimitive::Byteslice);
        a.unify(&mut b).unwrap();
    }

    #[test]
    fn test_simple_inference() {
        let mut a = TypeEnum::Var(TypeVar::new());
        let mut b = TypeEnum::Simple(TypePrimitive::UInt64);
        a.unify(&mut b).unwrap();
        match a {
            TypeEnum::Var(ref tv) => match **tv.value.borrow() {
                Some(TypeEnum::Simple(ref v)) if v == &TypePrimitive::UInt64 => {}
                None => panic!("Type variable should be set"),
                _ => panic!("Type variable set incorrectly"),
            },
            _ => panic!("Type variable should still be type variable"),
        };
    }

    #[test]
    fn complex_inference_bidirectional() {
        let mut a = TypeEnum::Arrow(
            Box::new(TypeEnum::Var(TypeVar::new())),
            Box::new(TypeEnum::Simple(TypePrimitive::Void)),
        );
        let mut b = TypeEnum::Arrow(
            Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
            Box::new(TypeEnum::Var(TypeVar::new())),
        );
        a.unify(&mut b).unwrap();
        println!("{:?}", a);
        println!("{:?}", b);
    }

    #[test]
    fn complex_inference_multi() {
        let tv = TypeVar::new();
        let mut a = TypeEnum::Arrow(
            Box::new(TypeEnum::Var(tv.clone())),
            Box::new(TypeEnum::Var(tv.clone())),
        );
        let mut b = TypeEnum::Arrow(
            Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
            Box::new(TypeEnum::Var(TypeVar::new())),
        );
        a.unify(&mut b).unwrap();
        println!("{:?}", a);
        println!("{:?}", b);
    }

    #[test]
    #[should_panic(expected = "UnresolvableTypeVariable")]
    fn complex_inference_recursive() {
        let tv = TypeVar::new();
        let mut a = TypeEnum::Arrow(
            Box::new(TypeEnum::Var(tv.clone())),
            Box::new(TypeEnum::Var(tv.clone())),
        );
        let mut b = TypeEnum::Var(tv.clone());
        a.unify(&mut b).unwrap();
        println!("{:?}", a);
        println!("{:?}", b);
    }

    #[test]
    fn type_display() {
        assert_eq!("int", TypeEnum::Simple(TypePrimitive::UInt64).to_string());
        assert_eq!("<void>", TypeEnum::Simple(TypePrimitive::Void).to_string());
        assert_eq!(
            "int -> int",
            TypeEnum::Arrow(
                Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
                Box::new(TypeEnum::Simple(TypePrimitive::UInt64)),
            )
            .to_string()
        );
        let tv = TypeVar::new();
        assert_eq!(
            "'a -> 'a",
            TypeEnum::Arrow(
                Box::new(TypeEnum::Var(tv.clone())),
                Box::new(TypeEnum::Var(tv.clone())),
            )
            .to_string()
        );
        let tv1 = TypeVar::new();
        let tv2 = TypeVar::new();
        assert_eq!(
            "'a -> 'b",
            TypeEnum::Arrow(
                Box::new(TypeEnum::Var(tv1.clone())),
                Box::new(TypeEnum::Var(tv2.clone())),
            )
            .to_string()
        );

        let tv1 = TypeVar::new();
        let tv2 = TypeVar::new();
        assert_eq!(
            "'a -> 'b -> 'a",
            TypeEnum::Arrow(
                Box::new(TypeEnum::Var(tv1.clone())),
                Box::new(TypeEnum::Arrow(
                    Box::new(TypeEnum::Var(tv2.clone())),
                    Box::new(TypeEnum::Var(tv1.clone())),
                )),
            )
            .to_string()
        );
    }
}
