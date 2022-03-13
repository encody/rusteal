// #![feature(trace_macros)]

// trace_macros!(true);

use crate::typing::TypeVar;

#[macro_export]
macro_rules! int {
    ($e: expr) => {
        Expr::Primitive(Primitive::UInt64($e))
    };
}

#[macro_export]
macro_rules! bytes {
    ($e: expr) => {
        Expr::Primitive(Primitive::Byteslice($e))
    };
}

#[macro_export]
macro_rules! void {
    () => {
        Expr::Primitive(Primitive::Void)
    };
}

#[macro_export]
macro_rules! seq {
    // Helper rules
    (@_opt) => { None };
    (@_opt $a:expr $(; $b:expr)* $(;)*) => {
        Some(Expr::Seq(Box::new(Seq(
            $a, seq!(@_opt $($b ;)*)
        ))))
    };

    ($a:expr $(; $b:expr)* $(;)*) => {
        Expr::Seq(Box::new(Seq(
            $a, seq!(@_opt $($b ;)*)
        )))
    };
}

#[macro_export]
macro_rules! apply {
    (@fn $f:expr ; ) => { $f };
    (@fn $f:expr ; @arg $a:expr $(;)*) => {
        Expr::Apply(Box::new(Apply(
            $f, $a
        )))
    };
    (@fn $f:expr ; @arg $a:expr $(; @arg $b:expr)+ $(;)*) => {
        apply!(
            @fn Expr::Apply(Box::new(Apply(
                $f, $a
            )))
            $(; @arg $b)+
        )
    };
}

#[macro_export]
macro_rules! binop {
    (($a:expr) == ($b:expr)) => {
        apply!(@fn Expr::Binary(Binary::Equals); @arg $b; @arg $a)
    };
    (($a:expr) != ($b:expr)) => {
        apply!(@fn Expr::Binary(Binary::NotEquals); @arg $b; @arg $a)
    };
    (($a:expr) > ($b:expr)) => {
        apply!(@fn Expr::Binary(Binary::GreaterThan); @arg $b; @arg $a)
    };
    (($a:expr) >= ($b:expr)) => {
        apply!(@fn Expr::Binary(Binary::GreaterThanEquals); @arg $b; @arg $a)
    };
    (($a:expr) < ($b:expr)) => {
        apply!(@fn Expr::Binary(Binary::LessThan); @arg $b; @arg $a)
    };
    (($a:expr) <= ($b:expr)) => {
        apply!(@fn Expr::Binary(Binary::LessThanEquals); @arg $b; @arg $a)
    };
}

#[macro_export]
macro_rules! bind_let {
    ($i:ident = $e:expr; $b:expr) => {
        Expr::Bind(Box::new(Bind::Let {
            identifier: String::from(stringify!($i)),
            value: $e,
            body: $b,
        }))
    };
}

#[macro_export]
macro_rules! assign {
    (@scratch $i:ident = $e:expr) => {
        apply!(
            @fn Expr::LVal(LVal(Var::Bind(String::from(stringify!($i)))));
            @arg $e;
        )
    };
    (@global $i:ident = $e:expr) => {
        apply!(
            @fn Expr::LVal(LVal(Var::Global(String::from(stringify!($i)))));
            @arg $e;
        )
    };
    (@local $i:ident[$who:expr] = $e:expr) => {
        apply!(
            @fn Expr::LVal(LVal(Var::Local(String::from(stringify!($i)))));
            @arg $who;
            @arg $e;
        )
    };
}

#[macro_export]
macro_rules! val {
    (@scratch $i:ident) => {
        Expr::RVal(RVal(Var::Bind(String::from(stringify!($i)))))
    };
    (@global $i:ident) => {
        Expr::RVal(RVal(Var::Global(String::from(stringify!($i)))))
    };
    (@local $i:ident[$who:expr]) => {
        apply!(
            @fn Expr::RVal(RVal(Var::Local(String::from(stringify!($i)))));
            @arg $who;
        )
    };
}

#[macro_export]
macro_rules! cond {
    // helpers
    (@_opt) => { None };
    (@_opt $a:expr => $e:expr $(; $b:expr => $be:expr)* $(;)*) => {
        Some(Box::new(Cond(
            $a,
            $e,
            cond!(@_opt $($b => $be;)*),
        )))
    };

    // main macro
    ($a:expr => $e:expr $(; $b:expr => $be:expr)* $(;)*) => {
        Expr::Cond(Box::new(Cond(
            $a,
            $e,
            cond!(@_opt $($b => $be;)*),
        )))
    }
}

#[macro_export]
macro_rules! r#if {
    (($e:expr) @then $true_expr:expr; @else $false_expr:expr $(;)*) => {
        apply!(
            @fn Expr::If(Box::new(If($true_expr, $false_expr)));
            @arg $e;
        )
    };
}

#[macro_export]
macro_rules! ret {
    ($e:expr) => {
        apply!(
            @fn Expr::Ret(Ret);
            @arg $e;
        )
    };
}

#[macro_export]
/// Type signatures may be written using the syntax:
/// 
/// ```rs
/// typesig!(<typesig>)
/// ```
/// 
/// where
/// 
/// ```txt
///     <typesig> ::
///           <simpletype>
///         | <simpletype> -> <typesig>
/// 
///     <simpletype> ::
///           <typekw>
///         | :<typevar>
/// 
///     <typekw> ::
///           int
///         | bytes
///         | void
///         | halt
/// 
///     <typevar> ::
///         (any identifier that does not match <typekw>)
/// ```
/// 
/// Examples:
/// 
/// ```rs
/// typesig!(int);
/// typesig!(bytes);
/// typesig!(bytes -> int);
/// typesig!(:a -> int);
/// typesig!(:a -> :a);
/// typesig!(:a -> :b);
/// typesig!(:a -> :b -> :a -> :c -> int);
/// ```
/// 
/// Function parameter syntax (e.g. `:a -> (:b -> :c) -> :d`) is not supported
macro_rules! typesig {
    ($(@_context $t:ident)? int) => {
        TypeEnum::Simple(TypePrimitive::UInt64)
    };
    ($(@_context $t:ident)? bytes) => {
        TypeEnum::Simple(TypePrimitive::Byteslice)
    };
    ($(@_context $t:ident)? void) => {
        TypeEnum::Simple(TypePrimitive::Void)
    };
    ($(@_context $t:ident)? halt) => {
        TypeEnum::Simple(TypePrimitive::Halt)
    };
    ($(:)? $a:ident) => {
        TypeEnum::Var(TypeVar::new())
    };
    (@get_tvar $t:ident $i:ident) => {
        {
            let i = stringify!($i);
            if $t.contains_key(&i) {
                TypeEnum::Var($t[&i].clone())
            } else {
                let tv = TypeVar::new();
                $t.insert(i, tv.clone());
                TypeEnum::Var(tv)
            }
        }
    };
    (@_context $t:ident $(:)? $a:ident) => {
        typesig!(@get_tvar $t $a)
    };
    (@_context $t:ident $(:)? $a:ident $($(->)? $(:)? $b:ident)+) => {
        TypeEnum::Arrow(
            Box::new(typesig!(@_context $t $a)),
            Box::new(typesig!(@_context $t $($b)+)),
        )
    };
    ($(:)? $a:ident $(-> $(:)? $b:ident)+) => {
        {
            // This is less efficient than writing the type by hand
            // Another possible option:
            // https://stackoverflow.com/a/64957454 (generate custom struct with fields for each unique tvar?)
            let mut _tvars: std::collections::HashMap<&'static str, TypeVar> = std::collections::HashMap::new();
            typesig!(@_context _tvars $a $($b)+)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::expression::apply::Apply;
    use crate::expression::binary::Binary;
    use crate::expression::bind::Bind;
    use crate::expression::cond::Cond;
    use crate::expression::if_else::If;
    use crate::expression::primitive::Primitive;
    use crate::expression::ret::Ret;
    use crate::expression::seq::Seq;
    use crate::expression::var::{LVal, RVal, Var};
    use crate::expression::{Expr, Expression};
    use crate::typing::{TypeEnum, TypeVar, TypePrimitive};

    #[test]
    fn test_typesig() {
        let mut t = typesig!(:a -> :a -> :b -> int);
        let mut t2 = typesig!(:c -> :c -> :asdf -> int);
        let expected = "'a -> 'a -> 'b -> int";
        println!("{}", t);
        println!("{}", t2);
        assert_eq!(format!("{}", t), expected);
        assert_eq!(format!("{}", t2), expected);
        t.unify(&mut t2).expect("Types are unifiable");
    }

    #[test]
    fn test() {
        // let x = seq!(int!(1) ; int!(3) ; int!(2));
        // println!("{:?}", x);

        // let x = apply!(@fn Expr::Binary(Binary::Equals); @arg int!(1); @arg int!(2));
        // println!("{:?}", x);

        // let x = binop!((int!(2)) > (int!(1)));
        // println!("{}", x.compile_raw().unwrap());

        let x = bind_let!(my_scratch = binop!((int!(2)) > (int!(1))); seq! {
            assign!(@local my_local[int!(0)] = int!(1));
            val!(@local my_local[int!(0)]);
            assign!(@global my_global = int!(2));
            val!(@global my_global);
            assign!(@scratch my_scratch = int!(2));
            val!(@scratch my_scratch);
            r#if!(
                (binop!((val!(@scratch my_scratch)) > (int!(4))))
                @then bytes!(">4".into());
                @else bytes!("<=4".into());
            );
            ret!(int!(1));
        });
        println!("{}", x.compile_raw().unwrap());

        // let x = cond!(
        //     binop!((int!(1)) == (int!(2))) => int!(0);
        //     binop!((int!(3)) == (int!(4))) => int!(0);
        //     binop!((int!(5)) == (int!(5))) => int!(1);
        // );
        // println!("{}", x.compile_raw().unwrap());
    }
}
