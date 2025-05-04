use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_object_freeze".as_bytes();
type SIGNATURE = fn(env: napi_env, object: napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_object_freeze(
  env: napi_env,
  object: napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, object) }
}
