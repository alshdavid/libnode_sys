use std::ffi::c_char;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "node_api_symbol_for".as_bytes();
type SIGNATURE =
  fn(env: napi_env, utf8name: *const c_char, length: isize, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn node_api_symbol_for(
  env: napi_env,
  utf8name: *const c_char,
  length: isize,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, utf8name, length, result)
  }
}
