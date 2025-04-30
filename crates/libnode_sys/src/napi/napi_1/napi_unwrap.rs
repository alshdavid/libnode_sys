use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_unwrap".as_bytes();
type SIGNATURE = fn(env: napi_env, js_object: napi_value, result: *mut *mut c_void) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_unwrap(
  env: napi_env,
  js_object: napi_value,
  result: *mut *mut c_void,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, js_object, result) }
}
