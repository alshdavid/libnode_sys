use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_is_promise".as_bytes();
type SIGNATURE = fn(env: napi_env, value: napi_value, is_promise: *mut bool) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_is_promise(
  env: napi_env,
  value: napi_value,
  is_promise: *mut bool,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, is_promise) }
}
