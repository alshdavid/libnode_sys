use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_call_threadsafe_function".as_bytes();
type SIGNATURE = fn(
  func: napi_threadsafe_function,
  data: *mut c_void,
  is_blocking: napi_threadsafe_function_call_mode,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_call_threadsafe_function(
  func: napi_threadsafe_function,
  data: *mut c_void,
  is_blocking: napi_threadsafe_function_call_mode,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(func, data, is_blocking) }
}
