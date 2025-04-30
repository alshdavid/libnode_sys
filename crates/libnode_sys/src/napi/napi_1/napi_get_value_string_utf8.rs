use std::ffi::c_char;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_value_string_utf8".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  value: napi_value,
  buf: *mut c_char,
  bufsize: usize,
  result: *mut usize,
) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_value_string_utf8(
  env: napi_env,
  value: napi_value,
  buf: *mut c_char,
  bufsize: usize,
  result: *mut usize,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, buf, bufsize, result)
  }
}
