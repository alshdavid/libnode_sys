use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_ref_threadsafe_function".as_bytes();
type SIGNATURE = fn(env: napi_env, func: napi_threadsafe_function) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_ref_threadsafe_function(
  env: napi_env,
  func: napi_threadsafe_function,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, func) }
}
