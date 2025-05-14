//! This crate contains the [`Resultish`] type, which represents success ([`Ok`]), failure
//! ([`Err`]), or [`Both`]. It can be converted into a [`Result`]:
//! - [`Resultish::lenient`]ly, where [`Both`] is mapped to [`Result::Ok`], and the
//!   failure value is discarded.
//! - [`Resultish::strict`]ly, where [`Both`] is mapped to [`Result::Err`], and the
//!   success value is discarded.

use Resultish::{Both, Err, Ok};

/// `Resultish` represents success ([`Ok`]), failure ([`Err`]), or [`Both`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[must_use]
pub enum Resultish<T, E> {
    /// Contains only a success value
    Ok(T),
    /// Contains only an error value
    Err(E),
    /// Contains a success and error value
    Both(T, E),
}

impl<T, E> Resultish<T, E> {
    pub fn has_ok(&self) -> bool {
        matches!(self, Ok(_) | Both(_, _))
    }

    pub fn has_err(&self) -> bool {
        matches!(self, Err(_) | Both(_, _))
    }

    pub fn lenient(self) -> Result<T, E> {
        match self {
            Ok(ok) => Result::Ok(ok),
            Err(err) => Result::Err(err),
            Both(ok, _) => Result::Ok(ok),
        }
    }

    pub fn lenient_err(self) -> Option<E> {
        match self {
            Ok(_) => None,
            Err(err) => Some(err),
            Both(_, _) => None,
        }
    }

    pub fn lenient_ok(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(_) => None,
            Both(ok, _) => Some(ok),
        }
    }

    pub fn map<U, F>(self, op: F) -> Resultish<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Ok(ok) => Ok(op(ok)),
            Err(err) => Err(err),
            Both(ok, err) => Both(op(ok), err),
        }
    }

    pub fn strict(self) -> Result<T, E> {
        match self {
            Ok(ok) => Result::Ok(ok),
            Err(err) => Result::Err(err),
            Both(_, err) => Result::Err(err),
        }
    }

    pub fn strict_err(self) -> Option<E> {
        match self {
            Ok(_) => None,
            Err(err) => Some(err),
            Both(_, err) => Some(err),
        }
    }

    pub fn strict_ok(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(_) => None,
            Both(_, _) => None,
        }
    }
}

impl<T, E> From<Result<T, E>> for Resultish<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Result::Ok(ok) => Ok(ok),
            Result::Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Resultish::{self, Both, Err, Ok};

    #[test]
    fn test_has() {
        let ok: Resultish<i32, String> = Ok(1);
        let err: Resultish<i32, String> = Err("hi".to_string());
        let both: Resultish<i32, String> = Both(1, "hi".to_string());
        assert_eq!(ok.has_ok(), true);
        assert_eq!(err.has_ok(), false);
        assert_eq!(both.has_ok(), true);

        assert_eq!(ok.has_err(), false);
        assert_eq!(err.has_err(), true);
        assert_eq!(both.has_err(), true);
    }
}
