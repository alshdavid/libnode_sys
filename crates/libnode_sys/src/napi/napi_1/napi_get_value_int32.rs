use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_value_int32".as_bytes();
type SIGNATURE = fn(env: napi_env, value: napi_value, result: *mut i32) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_value_int32(
  env: napi_env,
  value: napi_value,
  result: *mut i32,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, result) }
}
