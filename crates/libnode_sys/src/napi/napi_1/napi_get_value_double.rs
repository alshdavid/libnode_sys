use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_value_double".as_bytes();
type SIGNATURE = fn(env: napi_env, value: napi_value, result: *mut f64) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_value_double(
  env: napi_env,
  value: napi_value,
  result: *mut f64,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, result) }
}
