use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_delete_element".as_bytes();
type SIGNATURE =
  fn(env: napi_env, object: napi_value, index: u32, result: *mut bool) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_delete_element(
  env: napi_env,
  object: napi_value,
  index: u32,
  result: *mut bool,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, object, index, result) }
}
