#[cfg(unix)]
pub type Symbol<T> = libloading::os::unix::Symbol<T>;

#[cfg(windows)]
pub type Symbol<T> = libloading::os::windows::Symbol<T>;

#[cfg(unix)]
pub type Library = libloading::os::unix::Library;

#[cfg(windows)]
pub type Library = libloading::os::windows::Library;

static LIBNODE: std::sync::OnceLock<crate::Result<Library>> = std::sync::OnceLock::new();

/// Load Nodejs symbols for current process, used for n-api extensions
pub fn this() -> &'static crate::Result<Library> {
  #[cfg(unix)]
  return LIBNODE.get_or_init(|| Ok(Library::this()));
  #[cfg(windows)]
  return LIBNODE.get_or_init(|| Ok(Library::this().unwrap()));
}

/// Load "libnode" dynamic library, used for embedding Nodejs into Rust processes
pub fn cdylib<P: AsRef<std::path::Path>>(path: P) -> crate::Result<&'static Library> {
  let Ok(lib) = LIBNODE.get_or_init(|| match unsafe { Library::new(path.as_ref()) } {
    Ok(lib) => Ok(lib),
    Err(_) => Err(crate::Error::LibnodeFailedToLoad),
  }) else {
    return Err(crate::Error::LibnodeFailedToLoad);
  };

  Ok(lib)
}

/// Get a symbol from libnode, this must be run after the library has loaded
pub unsafe fn get_sym<T>(symbol: &[u8]) -> crate::Result<Symbol<T>> {
  let lib = match LIBNODE.get() {
    Some(Ok(lib)) => lib,
    Some(Err(err)) => return Err(crate::Error::from(err)),
    None => return Err(crate::Error::LibnodeNotLoaded),
  };
  match unsafe { lib.get(symbol.as_ref()) } {
    Ok(sym) => Ok(sym),
    Err(_) => Err(crate::Error::LibnodeSymbolNotFound),
  }
}
