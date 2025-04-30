use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_async_destroy".as_bytes();
type SIGNATURE = fn(env: napi_env, async_context: napi_async_context) -> napi_status;

static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_async_destroy(
  env: napi_env,
  async_context: napi_async_context,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, async_context) }
}
