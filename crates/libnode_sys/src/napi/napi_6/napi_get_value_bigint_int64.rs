use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_value_bigint_int64".as_bytes();
type SIGNATURE =
  fn(env: napi_env, value: napi_value, result: *mut i64, lossless: *mut bool) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_value_bigint_int64(
  env: napi_env,
  value: napi_value,
  result: *mut i64,
  lossless: *mut bool,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, result, lossless)
  }
}
