use std::ffi::c_char;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_throw_error".as_bytes();
type SIGNATURE = fn(env: napi_env, code: *const c_char, msg: *const c_char) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_throw_error(
  env: napi_env,
  code: *const c_char,
  msg: *const c_char,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, code, msg) }
}
