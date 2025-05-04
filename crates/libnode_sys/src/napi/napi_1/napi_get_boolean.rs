use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_boolean".as_bytes();
type SIGNATURE = fn(env: napi_env, value: bool, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_boolean(
  env: napi_env,
  value: bool,
  result: *mut napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, result) }
}
