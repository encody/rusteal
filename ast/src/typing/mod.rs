mod type_enum;
pub use type_enum::TypeEnum;
mod type_error;
pub use type_error::TypeError;
mod type_primitive;
pub use type_primitive::TypePrimitive;
mod type_var;
pub use type_var::TypeVar;

#[cfg(test)]
mod tests {
    use crate::typing::{type_enum::TypeEnum, type_primitive::TypePrimitive};

    use super::type_var::TypeVar;

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
