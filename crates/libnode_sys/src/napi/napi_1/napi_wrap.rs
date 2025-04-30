use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_wrap".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  js_object: napi_value,
  native_object: *mut c_void,
  finalize_cb: napi_finalize,
  finalize_hint: *mut c_void,
  result: *mut napi_ref,
) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_wrap(
  env: napi_env,
  js_object: napi_value,
  native_object: *mut c_void,
  finalize_cb: napi_finalize,
  finalize_hint: *mut c_void,
  result: *mut napi_ref,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      js_object,
      native_object,
      finalize_cb,
      finalize_hint,
      result,
    )
  }
}
