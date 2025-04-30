use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_create_async_work".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  async_resource: napi_value,
  async_resource_name: napi_value,
  execute: napi_async_execute_callback,
  complete: napi_async_complete_callback,
  data: *mut c_void,
  result: *mut napi_async_work,
) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_create_async_work(
  env: napi_env,
  async_resource: napi_value,
  async_resource_name: napi_value,
  execute: napi_async_execute_callback,
  complete: napi_async_complete_callback,
  data: *mut c_void,
  result: *mut napi_async_work,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env,
      async_resource,
      async_resource_name,
      execute,
      complete,
      data,
      result,
    )
  }
}
