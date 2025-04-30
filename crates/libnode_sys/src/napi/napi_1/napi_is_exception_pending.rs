use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_is_exception_pending".as_bytes();
type SIGNATURE = fn(env: napi_env, result: *mut bool) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_is_exception_pending(
  env: napi_env,
  result: *mut bool,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, result) }
}
