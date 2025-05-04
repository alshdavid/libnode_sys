use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_instance_data".as_bytes();
type SIGNATURE = fn(env: napi_env, data: *mut *mut c_void) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_instance_data(
  env: napi_env,
  data: *mut *mut c_void,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, data) }
}
