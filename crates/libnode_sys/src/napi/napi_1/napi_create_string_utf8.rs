use std::ffi::c_char;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_string_utf8".as_bytes();
type SIGNATURE =
  fn(env: napi_env, str_: *const c_char, length: isize, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_string_utf8(
  env: napi_env,
  str_: *const c_char,
  length: isize,
  result: *mut napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, str_, length, result) }
}
