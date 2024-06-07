#[macro_export]
macro_rules! match_arm {
    (numeric, $enum: ident, $e: ident, $body: tt) => {
        $crate::match_arm!($enum, $e, $body, F32, F64, I32, I64, U64, Usize, OptUsize)
    };
    ($enum: ident, $e: ident, $body: tt, $($(#[$meta: meta])? $arm: ident),*) => {
        $(
            $enum::$arm($e) => Ok($body),
        )*
    };
}

#[macro_export]
macro_rules! match_enum {
    // select the match arm
    ($enum: ident, $exprs: expr, $e: ident, $body: tt, $($(#[$meta: meta])? $arm: ident),* $(,)?) => {
        match $exprs {
            $($(#[$meta])? $enum::$arm($e) => Ok($body),)*
            _ => Err(terr!("Not supported arm for enum {:?}", stringify!($enum)))
            // _ => unimplemented!("Not supported dtype for {:?}", stringify!($e))
        }
    };

    ( $enum: ident, $expr:expr , $( $(#[$meta: meta])? $( $pat:pat_param )|+ => $expr_arm:expr ),+ ) => {
        {
            use $enum::*;
            match $expr {
                $(
                    $(#[$meta])?
                    $( $pat => Ok($expr_arm), )+
                )+
                _ => Err(terr!("Not supported arm in match enum"))
            }
        }
    };

    ($enum: ident, $exprs: expr, $e: ident, $body: tt) => {
        {
            $crate::match_enum!(
                $enum, $exprs, $e, $body,
                F32, F64, I32, I64, U8, U64, Bool, Usize, Str, String, Object, OptUsize, VecUsize,
                #[cfg(feature="time")] DateTime,
                #[cfg(feature="time")] TimeDelta,
            )
        }
    };

    ($enum: ident, ($exprs1: expr, $e1: ident, $($arm1: ident),*), ($exprs2: expr, $e2: ident, $($arm2: ident),*), $body: tt) => {
        $crate::match_enum!($enum, $exprs1, $e1, {$crate::match_enum!($enum, $exprs2, $e2, $body, $($arm2),*)}, $($arm1),*)
    };

    // match dtype that support dynamic(currently str and Object doesn't support)
    ($enum: ident, dynamic $($tt: tt)*) => {
        $crate::match_enum!($enum, $($tt)*, F32, F64, I32, I64, U8, U64, Bool, Usize, String, OptUsize, VecUsize,
        #[cfg(feature="time")] DateTime,
        #[cfg(feature="time")] TimeDelta,)
    };

    // match pure numeric dtype
    ($enum: ident, pure numeric $($tt: tt)*) => {
        $crate::match_enum!($enum, $($tt)*, F32, F64, I32, I64, U64, Usize)
    };

    // match numeric dtype
    ($enum: ident, numeric $($tt: tt)*) => {
        $crate::match_enum!($enum, $($tt)*, F32, F64, I32, I64, U64, Usize, OptUsize)
    };

    // match int like dtype
    ($enum: ident, int $($tt: tt)*) => {
        $crate::match_enum!($enum, $($tt)*, I32, I64, U64, Usize, OptUsize)
    };

    // match float like dtype
    ($enum: ident, float $($tt: tt)*) => {
        $crate::match_enum!($enum, $($tt)*, F32, F64)
    };

    // match bool like dtype
    ($enum: ident, bool $($tt: tt)*) => {
        $crate::match_enum!($enum, $($tt)*, Bool)
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

    // match non-reference dtype(no str)
    ($enum: ident, own $($tt: tt)*) => {
        $crate::match_enum!(
            $enum, $($tt)*,
            F32, F64, I32, I64, U64, Usize, OptUsize, VecUsize,
            Object, String, Bool, U8,
            #[cfg(feature="time")] DateTime,
            #[cfg(feature="time")] TimeDelta,
        )
    };

    // match dtype that can be used in python
    ($enum: ident, pyelement $($tt: tt)*) => {
        $crate::match_enum!(
            $enum, $($tt)*,
            F32, F64, I32, I64, U64, Usize, Bool, Object,
        )
    };
}
