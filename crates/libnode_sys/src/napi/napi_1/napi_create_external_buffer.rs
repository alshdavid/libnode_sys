use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_external_buffer".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  length: usize,
  data: *mut c_void,
  finalize_cb: napi_finalize,
  finalize_hint: *mut c_void,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_external_buffer(
  env: napi_env,
  length: usize,
  data: *mut c_void,
  finalize_cb: napi_finalize,
  finalize_hint: *mut c_void,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      length,
      data,
      finalize_cb,
      finalize_hint,
      result,
    )
  }
}
