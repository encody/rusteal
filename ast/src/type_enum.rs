use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeCheckError {
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeEnum {
    Simple(TypePrimitive),
    Arrow(Box<TypeEnum>, Box<TypeEnum>),
    Var(TypeVar),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypePrimitive {
    Void,
    UInt64,
    Byteslice,
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
    pub fn unify(&mut self, other: &mut Self) -> Result<(), TypeCheckError> {
        match (&mut self.clone(), &mut other.clone()) {
            (a, b) if a == b => Ok(()),
            (TypeEnum::Var(tv), _) => match ((*tv.value).borrow_mut()).as_mut() {
                Some(ref mut x) => x.unify(other),
                None if other.contains(tv) => Err(TypeCheckError::UnresolvableTypeVariable(
                    tv.clone(),
                    other.clone(),
                )),
                inner => {
                    *inner = Some(other.clone());
                    Ok(())
                }
            },
            (_, TypeEnum::Var(_)) => other.unify(self),
            (TypeEnum::Simple(a), TypeEnum::Simple(b)) if a == b => Ok(()),
            (TypeEnum::Arrow(ref mut a1, ref mut a2), TypeEnum::Arrow(ref mut b1, ref mut b2)) => {
                a1.unify(b1).and(a2.unify(b2))
            }
            (a, b) => Err(TypeCheckError::IrreconcilableTypes(a.clone(), b.clone())),
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
}
