use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_uv_event_loop".as_bytes();
type SIGNATURE = fn(env: napi_env, loop_: *mut *mut uv_loop_s) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_uv_event_loop(
  env: napi_env,
  loop_: *mut *mut uv_loop_s,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, loop_) }
}
