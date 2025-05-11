#[cfg(target_os = "linux")]
pub static EXT: &str = "so";

#[cfg(target_os = "macos")]
pub static EXT: &str = "dylib";

#[cfg(target_os = "windows")]
pub static EXT: &str = "dll";

#[cfg(target_os = "linux")]
pub static LIB_NAME: &str = "libnode.so";

#[cfg(target_os = "macos")]
pub static LIB_NAME: &str = "libnode.dylib";

#[cfg(target_os = "windows")]
pub static LIB_NAME: &str = "libnode.dll";

pub fn with_extension<S: AsRef<str>>(name: S) -> String {
  return format!("{}.{}", name.as_ref(), EXT);
}
