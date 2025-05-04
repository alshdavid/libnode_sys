pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
  LibnodeNotLoaded,
  LibnodeFailedToLoad,
  LibnodeSymbolNotFound,
}

impl std::fmt::Debug for Error {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::LibnodeNotLoaded => write!(f, "LibnodeNotLoaded"),
      Self::LibnodeFailedToLoad => write!(f, "LibnodeFailedToLoad"),
      Self::LibnodeSymbolNotFound => write!(f, "LibnodeSymbolNotFound"),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Error::LibnodeFailedToLoad => write!(f, "LibnodeFailedToLoad"),
      Error::LibnodeNotLoaded => write!(f, "LibnodeNotLoaded"),
      Error::LibnodeSymbolNotFound => write!(f, "LibnodeSymbolNotFound"),
    }
  }
}

impl std::error::Error for Error {}

impl From<&Error> for Error {
  fn from(value: &Error) -> Self {
    match value {
      Error::LibnodeNotLoaded => Error::LibnodeNotLoaded,
      Error::LibnodeFailedToLoad => Error::LibnodeFailedToLoad,
      Error::LibnodeSymbolNotFound => Error::LibnodeSymbolNotFound,
    }
  }
}
