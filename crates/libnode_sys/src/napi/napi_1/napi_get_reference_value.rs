use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_reference_value".as_bytes();
type SIGNATURE = fn(env: napi_env, ref_: napi_ref, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_reference_value(
  env: napi_env,
  ref_: napi_ref,
  result: *mut napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, ref_, result) }
}
