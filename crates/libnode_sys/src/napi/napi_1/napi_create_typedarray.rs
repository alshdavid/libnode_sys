use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_typedarray".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  type_: napi_typedarray_type,
  length: usize,
  arraybuffer: napi_value,
  byte_offset: usize,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_typedarray(
  env: napi_env,
  type_: napi_typedarray_type,
  length: usize,
  arraybuffer: napi_value,
  byte_offset: usize,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      type_,
      length,
      arraybuffer,
      byte_offset,
      result,
    )
  }
}
