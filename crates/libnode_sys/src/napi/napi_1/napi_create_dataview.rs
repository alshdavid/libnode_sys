use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_dataview".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  length: usize,
  arraybuffer: napi_value,
  byte_offset: usize,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_dataview(
  env: napi_env,
  length: usize,
  arraybuffer: napi_value,
  byte_offset: usize,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      length,
      arraybuffer,
      byte_offset,
      result,
    )
  }
}
