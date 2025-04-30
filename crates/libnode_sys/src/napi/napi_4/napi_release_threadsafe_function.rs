use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_release_threadsafe_function".as_bytes();
type SIGNATURE =
  fn(func: napi_threadsafe_function, mode: napi_threadsafe_function_release_mode) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_release_threadsafe_function(
  func: napi_threadsafe_function,
  mode: napi_threadsafe_function_release_mode,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(func, mode) }
}
