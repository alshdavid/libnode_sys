use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_open_handle_scope".as_bytes();
type SIGNATURE = fn(env: napi_env, result: *mut napi_handle_scope) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_open_handle_scope(
  env: napi_env,
  result: *mut napi_handle_scope,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, result) }
}
