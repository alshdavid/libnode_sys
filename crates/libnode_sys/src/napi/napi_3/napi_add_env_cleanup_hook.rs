use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_add_env_cleanup_hook".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  fun: Option<unsafe extern "C" fn(arg: *mut c_void)>,
  arg: *mut c_void,
) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_add_env_cleanup_hook(
  env: napi_env,
  fun: Option<unsafe extern "C" fn(arg: *mut c_void)>,
  arg: *mut c_void,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, fun, arg) }
}
