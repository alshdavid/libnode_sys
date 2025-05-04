use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "node_api_create_syntax_error".as_bytes();
type SIGNATURE =
  fn(env: napi_env, code: napi_value, msg: napi_value, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn node_api_create_syntax_error(
  env: napi_env,
  code: napi_value,
  msg: napi_value,
  result: *mut napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, code, msg, result) }
}
