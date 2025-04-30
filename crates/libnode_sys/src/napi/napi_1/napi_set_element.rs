use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_set_element".as_bytes();
type SIGNATURE =
  fn(env: napi_env, object: napi_value, index: u32, value: napi_value) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_set_element(
  env: napi_env,
  object: napi_value,
  index: u32,
  value: napi_value,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, object, index, value) }
}
