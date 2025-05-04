use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_buffer_copy".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  length: usize,
  data: *const c_void,
  result_data: *mut *mut c_void,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_buffer_copy(
  env: napi_env,
  length: usize,
  data: *const c_void,
  result_data: *mut *mut c_void,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      length,
      data,
      result_data,
      result,
    )
  }
}
