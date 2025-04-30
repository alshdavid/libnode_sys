use std::ffi::c_char;
use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_function".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  utf8name: *const c_char,
  length: isize,
  cb: napi_callback,
  data: *mut c_void,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_function(
  env: napi_env,
  utf8name: *const c_char,
  length: isize,
  cb: napi_callback,
  data: *mut c_void,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env, utf8name, length, cb, data, result,
    )
  }
}
