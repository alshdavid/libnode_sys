use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_arraybuffer".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  byte_length: usize,
  data: *mut *mut c_void,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_arraybuffer(
  env: napi_env,
  byte_length: usize,
  data: *mut *mut c_void,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, byte_length, data, result)
  }
}
