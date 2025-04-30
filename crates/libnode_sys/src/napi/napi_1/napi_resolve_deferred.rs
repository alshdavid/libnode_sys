use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_resolve_deferred".as_bytes();
type SIGNATURE = fn(env: napi_env, deferred: napi_deferred, resolution: napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_resolve_deferred(
  env: napi_env,
  deferred: napi_deferred,
  resolution: napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, deferred, resolution) }
}
