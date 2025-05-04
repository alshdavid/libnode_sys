use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_reference_unref".as_bytes();
type SIGNATURE = fn(env: napi_env, ref_: napi_ref, result: *mut u32) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_reference_unref(
  env: napi_env,
  ref_: napi_ref,
  result: *mut u32,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, ref_, result) }
}
