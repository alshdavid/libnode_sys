use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_bigint_uint64".as_bytes();
type SIGNATURE = fn(env: napi_env, value: u64, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_bigint_uint64(
  env: napi_env,
  value: u64,
  result: *mut napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, result) }
}
