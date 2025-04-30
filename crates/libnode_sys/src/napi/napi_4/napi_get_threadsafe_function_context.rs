use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_threadsafe_function_context".as_bytes();
type SIGNATURE = fn(func: napi_threadsafe_function, result: *mut *mut c_void) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_threadsafe_function_context(
  func: napi_threadsafe_function,
  result: *mut *mut c_void,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(func, result) }
}
