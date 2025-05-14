use Resultish::{Both, Err, Ok};

/// `Resultish` represents success ([`Ok`]), failure ([`Err`]), or [`Both`].
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[must_use]
pub enum Resultish<T, E> {
    /// Contains only a success value
    Ok(T),
    /// Contains a success and error value
    Both(T, E),
    /// Contains only an error value
    Err(E),
}

impl<T, E> Resultish<T, E> {
    pub fn has_ok(&self) -> bool {
        matches!(self, Ok(_) | Both(_, _))
    }

    pub fn has_err(&self) -> bool {
        matches!(self, Both(_, _) | Err(_))
    }

    pub fn lenient(self) -> Result<T, E> {
        match self {
            Ok(ok) => Result::Ok(ok),
            Both(ok, _) => Result::Ok(ok),
            Err(err) => Result::Err(err),
        }
    }

    pub fn lenient_err(self) -> Option<E> {
        match self {
            Ok(_) => None,
            Both(_, _) => None,
            Err(err) => Some(err),
        }
    }

    pub fn lenient_ok(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Both(ok, _) => Some(ok),
            Err(_) => None,
        }
    }

    pub fn map<U, F>(self, op: F) -> Resultish<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Ok(ok) => Ok(op(ok)),
            Both(ok, err) => Both(op(ok), err),
            Err(err) => Err(err),
        }
    }

    pub fn strict(self) -> Result<T, E> {
        match self {
            Ok(ok) => Result::Ok(ok),
            Both(_, err) => Result::Err(err),
            Err(err) => Result::Err(err),
        }
    }

    pub fn strict_err(self) -> Option<E> {
        match self {
            Ok(_) => None,
            Both(_, err) => Some(err),
            Err(err) => Some(err),
        }
    }

    pub fn strict_ok(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Both(_, _) => None,
            Err(_) => None,
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
        let both: Resultish<i32, String> = Both(1, "hi".to_string());
        let err: Resultish<i32, String> = Err("hi".to_string());
        assert_eq!(ok.has_ok(), true);
        assert_eq!(both.has_ok(), true);
        assert_eq!(err.has_ok(), false);

        assert_eq!(ok.has_err(), false);
        assert_eq!(both.has_err(), true);
        assert_eq!(err.has_err(), true);
    }
}
