use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_detach_arraybuffer".as_bytes();
type SIGNATURE = fn(env: napi_env, arraybuffer: napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_detach_arraybuffer(
  env: napi_env,
  arraybuffer: napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, arraybuffer) }
}
