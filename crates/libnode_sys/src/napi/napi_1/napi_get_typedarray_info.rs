use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_typedarray_info".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  typedarray: napi_value,
  type_: *mut napi_typedarray_type,
  length: *mut usize,
  data: *mut *mut c_void,
  arraybuffer: *mut napi_value,
  byte_offset: *mut usize,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_typedarray_info(
  env: napi_env,
  typedarray: napi_value,
  type_: *mut napi_typedarray_type,
  length: *mut usize,
  data: *mut *mut c_void,
  arraybuffer: *mut napi_value,
  byte_offset: *mut usize,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      typedarray,
      type_,
      length,
      data,
      arraybuffer,
      byte_offset,
    )
  }
}
