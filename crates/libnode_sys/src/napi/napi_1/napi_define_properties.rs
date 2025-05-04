use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_define_properties".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  object: napi_value,
  property_count: usize,
  properties: *const napi_property_descriptor,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_define_properties(
  env: napi_env,
  object: napi_value,
  property_count: usize,
  properties: *const napi_property_descriptor,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      object,
      property_count,
      properties,
    )
  }
}
