use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_coerce_to_bool".as_bytes();
type SIGNATURE = fn(env: napi_env, value: napi_value, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_coerce_to_bool(
  env: napi_env,
  value: napi_value,
  result: *mut napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, result) }
}
