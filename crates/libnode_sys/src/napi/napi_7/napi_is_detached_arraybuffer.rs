use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_is_detached_arraybuffer".as_bytes();
type SIGNATURE = fn(env: napi_env, value: napi_value, result: *mut bool) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_is_detached_arraybuffer(
  env: napi_env,
  value: napi_value,
  result: *mut bool,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, result) }
}
