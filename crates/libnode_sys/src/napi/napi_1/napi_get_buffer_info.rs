use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_buffer_info".as_bytes();
type SIGNATURE =
  fn(env: napi_env, value: napi_value, data: *mut *mut c_void, length: *mut usize) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_buffer_info(
  env: napi_env,
  value: napi_value,
  data: *mut *mut c_void,
  length: *mut usize,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, data, length) }
}
