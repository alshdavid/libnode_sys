use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_promise".as_bytes();
type SIGNATURE =
  fn(env: napi_env, deferred: *mut napi_deferred, promise: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_promise(
  env: napi_env,
  deferred: *mut napi_deferred,
  promise: *mut napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, deferred, promise) }
}
