use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_all_property_names".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  object: napi_value,
  key_mode: napi_key_collection_mode,
  key_filter: napi_key_filter,
  key_conversion: napi_key_conversion,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_all_property_names(
  env: napi_env,
  object: napi_value,
  key_mode: napi_key_collection_mode,
  key_filter: napi_key_filter,
  key_conversion: napi_key_conversion,
  result: *mut napi_value,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      object,
      key_mode,
      key_filter,
      key_conversion,
      result,
    )
  }
}
