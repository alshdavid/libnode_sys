use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_reject_deferred".as_bytes();
type SIGNATURE = fn(env: napi_env, deferred: napi_deferred, rejection: napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_reject_deferred(
  env: napi_env,
  deferred: napi_deferred,
  rejection: napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, deferred, rejection) }
}
