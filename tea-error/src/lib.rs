use std::{borrow::Cow, ops::Deref};

pub use anyhow::{anyhow, bail, ensure, Result};

use thiserror::Error;

#[cfg(feature = "polars")]
use polars::prelude::PolarsError;

#[derive(Debug)]
pub struct ErrInfo(Cow<'static, str>);

impl<T> From<T> for ErrInfo
where
    T: Into<Cow<'static, str>>,
{
    #[inline]
    fn from(msg: T) -> Self {
        Self(msg.into())
    }
}

impl AsRef<str> for ErrInfo {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ErrInfo {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ErrInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug)]
pub enum TError {
    #[error("The length of both vec doesn't match, left: {left} right: {right}")]
    LengthMismatch { left: usize, right: usize },
    #[error("{0}")]
    Str(ErrInfo),
    #[error("unknown error")]
    Unknown,
}

pub type TResult<T> = Result<T, TError>;

#[macro_export]
macro_rules! terr {
    ($variant:ident: $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__private::must_use(
            $crate::TError::$variant(format!($fmt, $($arg),*).into())
        )
    };
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::__private::must_use(
            $crate::TError::Str(format!($fmt, $($arg),*).into())
        )
    };
    ($variant: ident: $err: expr $(,)?) => {
        $crate::__private::must_use(
            $crate::TError::$variant($err.into())
        )
    };
    ($err: expr) => {
        $crate::TError::Str($err.into())
    };
    (lm, $left: expr, $right: expr) => {
        $crate::TError::LengthMismatch {
            left: $left,
            right: $right,
        }
    };
    () => {
        $crate::TError::Unknown
    };
}

#[macro_export]
macro_rules! tbail {
    ($($tt:tt)+) => {
        return Err($crate::terr!($($tt)+))
    };
}

#[macro_export]
macro_rules! tensure {
    ($cond:expr, $($tt:tt)+) => {
        if !$cond {
            $crate::tbail!($($tt)+);
        }
    };
}

#[cfg(feature = "polars")]
impl From<TError> for PolarsError {
    fn from(e: TError) -> Self {
        PolarsError::ComputeError(format!("{}", e).into())
    }
}

// Not public, referenced by macros only.
#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    #[inline]
    #[cold]
    #[must_use]
    pub fn must_use(error: crate::TError) -> crate::TError {
        error
    }
}
