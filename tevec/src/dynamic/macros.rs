#[macro_export]
macro_rules! match_enum {
    // select the match arm
    ($enum: ident, $exprs: expr, $e: ident, $body: tt, $($(#[$meta: meta])? $arm: ident),* $(,)?) => {
        match $exprs {
            $($(#[$meta])? $enum::$arm($e) => $body,)*
            _ => Err(terr!("Not supported arm for enum {:?}", stringify!($enum)))
        }
    };

    ($enum: ident, $exprs: expr; $($rest: tt)*) => {
        $crate::match_enum!(@($enum, $exprs; $($rest)*))
    };

    // match all arm
    (@($enum: ident, $exprs: expr; all ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            F32($e) => $body,
            F64($e) => $body,
            I32($e) => $body,
            I64($e) => $body,
            U8($e) => $body,
            U64($e) => $body,
            Bool($e) => $body,
            Usize($e) => $body,
            Str($e) => $body,
            String($e) => $body,
            Object($e) => $body,
            OptUsize($e) => $body,
            VecUsize($e) => $body,
            #[cfg(feature="time")] DateTime($e) => $body,
            #[cfg(feature="time")] TimeDelta($e) => $body,
        )
    };

    // match castable arm
    (@($enum: ident, $exprs: expr; cast ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            F32($e) => $body,
            F64($e) => $body,
            I32($e) => $body,
            I64($e) => $body,
            U8($e) => $body,
            U64($e) => $body,
            Bool($e) => $body,
            Usize($e) => $body,
            String($e) => $body,
            // Object($e) => $body,
            OptUsize($e) => $body,
            #[cfg(feature="time")] DateTime($e) => $body,
            #[cfg(feature="time")] TimeDelta($e) => $body,
        )
    };

    // match non-reference dtype(no str)
    (@($enum: ident, $exprs: expr; own ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            F32($e) => $body,
            F64($e) => $body,
            I32($e) => $body,
            I64($e) => $body,
            U8($e) => $body,
            U64($e) => $body,
            Bool($e) => $body,
            Usize($e) => $body,
            String($e) => $body,
            Object($e) => $body,
            OptUsize($e) => $body,
            VecUsize($e) => $body,
            #[cfg(feature="time")] DateTime($e) => $body,
            #[cfg(feature="time")] TimeDelta($e) => $body,
        )
    };

    // match dtype that can be used in python
    (@($enum: ident, $exprs: expr; py ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            F32($e) => $body,
            F64($e) => $body,
            I32($e) => $body,
            I64($e) => $body,
            U8($e) => $body,
            U64($e) => $body,
            Bool($e) => $body,
            Usize($e) => $body,
            Object($e) => $body,
        )
    };

    // match dtype that support dynamic(currently str and Object doesn't support)
    (@($enum: ident, $exprs: expr; dynamic ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            F32($e) => $body,
            F64($e) => $body,
            I32($e) => $body,
            I64($e) => $body,
            U8($e) => $body,
            U64($e) => $body,
            Bool($e) => $body,
            Usize($e) => $body,
            String($e) => $body,
            OptUsize($e) => $body,
            VecUsize($e) => $body,
            #[cfg(feature="time")] DateTime($e) => $body,
            #[cfg(feature="time")] TimeDelta($e) => $body,
        )
    };

    // match int like arm
    (@($enum: ident, $exprs: expr; int ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            I32($e) => $body,
            I64($e) => $body,
            U64($e) => $body,
            Usize($e) => $body,
            OptUsize($e) => $body,
        )
    };

    // match float like arm
    (@($enum: ident, $exprs: expr; float ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            F32($e) => $body,
            F64($e) => $body,
        )
    };

    // match pure numeric arm
    (@($enum: ident, $exprs: expr; pure_numeric ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            F32($e) => $body,
            F64($e) => $body,
            I32($e) => $body,
            I64($e) => $body,
            U64($e) => $body,
            Usize($e) => $body,
        )
    };

    // match numeric arm
    (@($enum: ident, $exprs: expr; numeric ($e: ident) => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            F32($e) => $body,
            F64($e) => $body,
            I32($e) => $body,
            I64($e) => $body,
            U64($e) => $body,
            Usize($e) => $body,
            OptUsize($e) => $body,
        )
    };

    // match one arm, note that this rule should be the last one
    (@($enum: ident, $exprs: expr; $($(#[$meta: meta])? $arms: ident ($e: ident))|+ => $body: expr, $($rest: tt)*) $($all_arms: tt)* ) => {
        $crate::match_enum!(
            @($enum, $exprs; $($rest)*)
            $($all_arms)*
            $($(#[$meta])? $arms($e) => $body,)*
        )
    };

    // No more match arms, produce final output
    (@($enum: ident, $exprs: expr; $(,)?) $($all_arms: tt)*) => {
        {
            use $enum::*;
            match $exprs {
                $($all_arms)*
                _ => Err(terr!("Not supported arm for enum {:?}", stringify!($enum)))
            }
        }
    };

    ($enum: ident, ($exprs1: expr, $e1: ident, $($arm1: ident),*), ($exprs2: expr, $e2: ident, $($arm2: ident),*), $body: tt) => {
        $crate::match_enum!($enum, $exprs1, $e1, {$crate::match_enum!($enum, $exprs2, $e2, $body, $($arm2),*)}, $($arm1),*)
    };

    // match hashable dtype
    ($enum: ident, hash $($tt: tt)*) => {
        $crate::match_enum!(
            $enum, $($tt)*,
             F32, F64, I32, I64, U64, Usize,
             String, Str, Bool, U8,
             #[cfg(feature="time")] DateTime
        )
    };
}
