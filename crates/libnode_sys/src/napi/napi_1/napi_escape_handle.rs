use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_escape_handle".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  scope: napi_escapable_handle_scope,
  escapee: napi_value,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_escape_handle(
  env: napi_env,
  scope: napi_escapable_handle_scope,
  escapee: napi_value,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, scope, escapee, result)
  }
}
