use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_last_error_info".as_bytes();
type SIGNATURE = fn(env: napi_env, result: *mut *const napi_extended_error_info) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_last_error_info(
  env: napi_env,
  result: *mut *const napi_extended_error_info,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, result) }
}
