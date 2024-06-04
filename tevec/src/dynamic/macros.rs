#[macro_export]
macro_rules! match_enum {
    // select the match arm
    ($enum: ident, $exprs: expr, $e: ident, $body: tt, $($(#[$meta: meta])? $arm: ident),* $(,)?) => {
        match $exprs {
            $($(#[$meta])? $enum::$arm($e) => $body,)*
            _ => unimplemented!("Not supported dtype for {:?}", stringify!($e))
        }
    };

    ($enum: ident, $exprs: expr, $e: ident, $body: tt) => {
        {
            match_enum!(
                $enum, $exprs, $e, $body,
                F32, F64, I32, I64, U8, U64, Bool, Usize, Str, String, Object, OptUsize, VecUsize,
                #[cfg(feature="time")] DateTime,
                #[cfg(feature="time")] TimeDelta,
            )
        }
    };

    ($enum: ident, ($exprs1: expr, $e1: ident, $($arm1: ident),*), ($exprs2: expr, $e2: ident, $($arm2: ident),*), $body: tt) => {
        match_enum!($enum, $exprs1, $e1, {match_enum!($enum, $exprs2, $e2, $body, $($arm2),*)}, $($arm1),*)
    };

    ($enum: ident, numeric $($tt: tt)*) => {
        match_enum!($enum, $($tt)*, F32, F64, I32, I64, U64, Usize, OptUsize)
    }
}

// #[macro_export]
// macro_rules! match_arrok {
//     (numeric $($tt: tt)*) => {
//         match_all!(ArrOk, $($tt)*,
//             F32, F64, I32, I64, U64, Usize,
//             // OptUsize,
//         )
//     };
//     (int $($tt: tt)*) => {match_all!(ArrOk, $($tt)*, I32, I64, Usize)};
//     (float $($tt: tt)*) => {match_all!(ArrOk, $($tt)*, F32, F64)};
//     (bool $($tt: tt)*) => {match_all!(ArrOk, $($tt)*, Bool)};
//     (hash $($tt: tt)*) => {match_all!(ArrOk, $($tt)*, I32, I64, U64, Usize, String, Str, #[cfg(feature="time")] DateTime, Bool, U8, U64)};
//     (tphash $($tt: tt)*) => {match_all!(ArrOk, $($tt)*, F32, F64, I32, I64, U64, Usize, String, Str, #[cfg(feature="time")] DateTime, Bool, U8, U64)};
//     (castable $($tt: tt)*) => {match_all!(
//         ArrOk, $($tt)*,
//         F32, F64, I32, I64, U64, Usize, String,
//         Bool, OptUsize,
//         #[cfg(feature="time")] DateTime,
//         #[cfg(feature="time")] TimeDelta,
//     )};
//     (nostr $($tt: tt)*) => {match_all!(
//         ArrOk, $($tt)*,
//         F32, F64, I32, I64, U64, Usize, String, U8, Bool, OptUsize, VecUsize, Object,
//         #[cfg(feature="time")] DateTime,
//         #[cfg(feature="time")] TimeDelta,
//     )};
//     (pyelement $($tt: tt)*) => {match_all!(ArrOk, $($tt)*, F32, F64, I32, I64, U64, Usize, Bool, Object)};
//     ($($tt: tt)*) => {match_all!(ArrOk, $($tt)*)};
// }
