use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_instanceof".as_bytes();
type SIGNATURE =
  fn(env: napi_env, object: napi_value, constructor: napi_value, result: *mut bool) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_instanceof(
  env: napi_env,
  object: napi_value,
  constructor: napi_value,
  result: *mut bool,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, object, constructor, result)
  }
}
