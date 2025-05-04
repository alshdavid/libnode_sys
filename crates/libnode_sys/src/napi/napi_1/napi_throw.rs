use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_throw".as_bytes();
type SIGNATURE = fn(env: napi_env, error: napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_throw(
  env: napi_env,
  error: napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, error) }
}
