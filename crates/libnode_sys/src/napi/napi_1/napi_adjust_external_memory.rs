use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_adjust_external_memory".as_bytes();
type SIGNATURE = fn(env: napi_env, change_in_bytes: i64, adjusted_value: *mut i64) -> napi_status;

static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_adjust_external_memory(
  env: napi_env,
  change_in_bytes: i64,
  adjusted_value: *mut i64,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      change_in_bytes,
      adjusted_value,
    )
  }
}
