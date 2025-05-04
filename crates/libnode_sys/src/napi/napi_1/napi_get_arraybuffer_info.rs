use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_arraybuffer_info".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  arraybuffer: napi_value,
  data: *mut *mut c_void,
  byte_length: *mut usize,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_arraybuffer_info(
  env: napi_env,
  arraybuffer: napi_value,
  data: *mut *mut c_void,
  byte_length: *mut usize,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, arraybuffer, data, byte_length)
  }
}
