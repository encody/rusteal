// #![feature(trace_macros)]

// trace_macros!(true);

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

#[cfg(test)]
mod tests {
    use crate::expression::apply::Apply;
    use crate::expression::binary::Binary;
    use crate::expression::bind::Bind;
    use crate::expression::cond::Cond;
    use crate::expression::if_else::If;
    use crate::expression::primitive::Primitive;
    use crate::expression::seq::Seq;
    use crate::expression::var::{LVal, RVal, Var};
    use crate::expression::{Expr, Expression};

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
