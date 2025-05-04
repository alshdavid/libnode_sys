use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_uint32".as_bytes();
type SIGNATURE = fn(env: napi_env, value: u32, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_uint32(
  env: napi_env,
  value: u32,
  result: *mut napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, value, result) }
}
