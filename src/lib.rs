//! A [`Resultish`] represents success ([`Ok`]), error ([`Err`]), or [`Both`]. It can be
//! converted into a [`Result`]:
//! - [`Resultish::lenient`]ly, where [`Both`] is mapped to [`Result::Ok`], and the
//!   error value is discarded.
//! - [`Resultish::strict`]ly, where [`Both`] is mapped to [`Result::Err`], and the
//!   success value is discarded.

use Resultish::{Both, Err, Ok};

/// `Resultish` represents success ([`Ok`]), error ([`Err`]), or [`Both`].
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
    /// Converts from `&mut Resultish<T, E>` to `Resultish<&mut T, &mut E>`.
    pub fn as_mut(&mut self) -> Resultish<&mut T, &mut E> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err),
            Both(ok, err) => Both(ok, err),
        }
    }

    /// Converts from `&Resultish<T, E>` to `Resultish<&T, &E>`.
    pub fn as_ref(&self) -> Resultish<&T, &E> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err),
            Both(ok, err) => Both(ok, err),
        }
    }

    /// Returns `true` if the result contains a success value.
    ///
    /// # Examples
    ///
    /// ```
    /// use resultish::Resultish::{self, Both, Err, Ok};
    ///
    /// let x: Resultish<i32, &str> = Ok(3);
    /// assert_eq!(x.has_ok(), true);
    ///
    /// let x: Resultish<i32, &str> = Err("Some error message");
    /// assert_eq!(x.has_ok(), false);
    ///
    /// let x: Resultish<i32, &str> = Both(3, "Some error message");
    /// assert_eq!(x.has_ok(), true);
    /// ```
    pub fn has_ok(&self) -> bool {
        matches!(self, Ok(_) | Both(_, _))
    }

    /// Returns `true` if the result contains an error value.
    ///
    /// # Examples
    ///
    /// ```
    /// use resultish::Resultish::{self, Both, Err, Ok};
    ///
    /// let x: Resultish<i32, &str> = Ok(3);
    /// assert_eq!(x.has_err(), false);
    ///
    /// let x: Resultish<i32, &str> = Err("Some error message");
    /// assert_eq!(x.has_err(), true);
    ///
    /// let x: Resultish<i32, &str> = Both(3, "Some error message");
    /// assert_eq!(x.has_err(), true);
    /// ```
    pub fn has_err(&self) -> bool {
        matches!(self, Err(_) | Both(_, _))
    }

    /// Convert to [`Result`] leniently: [`Both`] is mapped to [`Result::Ok`], and the error value
    /// is discarded.
    ///
    /// # Examples
    ///
    /// ```
    /// use resultish::Resultish::{self, Both, Err, Ok};
    ///
    /// let x: Resultish<i32, &str> = Ok(3);
    /// assert_eq!(x.lenient(), Result::Ok(3));
    ///
    /// let x: Resultish<i32, &str> = Err("Some error message");
    /// assert_eq!(x.lenient(), Result::Err("Some error message"));
    ///
    /// let x: Resultish<i32, &str> = Both(3, "Some error message");
    /// assert_eq!(x.lenient(), Result::Ok(3));
    /// ```
    pub fn lenient(self) -> Result<T, E> {
        match self {
            Ok(ok) => Result::Ok(ok),
            Err(err) => Result::Err(err),
            Both(ok, _) => Result::Ok(ok),
        }
    }

    /// Equivalent to [`lenient`](`Self::lenient`)`().`[`err`](Result::err)`()`.
    pub fn lenient_err(self) -> Option<E> {
        match self {
            Ok(_) => None,
            Err(err) => Some(err),
            Both(_, _) => None,
        }
    }

    /// Equivalent to [`lenient`](`Self::lenient`)`().`[`ok`](Result::ok)`()`.
    pub fn lenient_ok(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(_) => None,
            Both(ok, _) => Some(ok),
        }
    }

    /// Maps a `Resultish<T, E>` to `Resultish<U, E>` by applying a function to the success value,
    /// and leaving the error value untouched
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

    /// Maps a `Resultish<T, E>` to `Resultish<T, F>` by applying a function to the success value,
    /// and leaving the error value untouched
    pub fn map_err<F, O>(self, op: O) -> Resultish<T, F>
    where
        O: FnOnce(E) -> F,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(op(err)),
            Both(ok, err) => Both(ok, op(err)),
        }
    }

    /// Convert to [`Result`] strictly: [`Both`] is mapped to [`Result::Err`], and the success value
    /// is discarded.
    ///
    /// # Examples
    ///
    /// ```
    /// use resultish::Resultish::{self, Both, Err, Ok};
    ///
    /// let x: Resultish<i32, &str> = Ok(3);
    /// assert_eq!(x.strict(), Result::Ok(3));
    ///
    /// let x: Resultish<i32, &str> = Err("Some error message");
    /// assert_eq!(x.strict(), Result::Err("Some error message"));
    ///
    /// let x: Resultish<i32, &str> = Both(3, "Some error message");
    /// assert_eq!(x.strict(), Result::Err("Some error message"));
    /// ```
    pub fn strict(self) -> Result<T, E> {
        match self {
            Ok(ok) => Result::Ok(ok),
            Err(err) => Result::Err(err),
            Both(_, err) => Result::Err(err),
        }
    }

    /// Equivalent to [`strict`](`Self::strict`)`().`[`err`](Result::err)`()`.
    pub fn strict_err(self) -> Option<E> {
        match self {
            Ok(_) => None,
            Err(err) => Some(err),
            Both(_, err) => Some(err),
        }
    }

    /// Equivalent to [`strict`](`Self::strict`)`().`[`ok`](Result::ok)`()`.
    pub fn strict_ok(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(_) => None,
            Both(_, _) => None,
        }
    }

    /// Convert to tuple of the success and error values.
    ///
    /// # Examples
    ///
    /// ```
    /// use resultish::Resultish::{self, Both, Err, Ok};
    ///
    /// let x: Resultish<i32, &str> = Ok(3);
    /// assert_eq!(x.tuple(), (Some(3), None));
    ///
    /// let x: Resultish<i32, &str> = Err("Some error message");
    /// assert_eq!(x.tuple(), (None, Some("Some error message")));
    ///
    /// let x: Resultish<i32, &str> = Both(3, "Some error message");
    /// assert_eq!(x.tuple(), (Some(3), Some("Some error message")));
    /// ```
    pub fn tuple(self) -> (Option<T>, Option<E>) {
        match self {
            Ok(ok) => (Some(ok), None),
            Err(err) => (None, Some(err)),
            Both(ok, err) => (Some(ok), Some(err)),
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
