use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_call_function".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  recv: napi_value,
  func: napi_value,
  argc: usize,
  argv: *const napi_value,
  result: *mut napi_value,
) -> napi_status;

static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_call_function(
  env: napi_env,
  recv: napi_value,
  func: napi_value,
  argc: usize,
  argv: *const napi_value,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, recv, func, argc, argv, result)
  }
}
