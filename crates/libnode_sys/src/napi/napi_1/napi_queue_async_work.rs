use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_queue_async_work".as_bytes();
type SIGNATURE = fn(env: napi_env, work: napi_async_work) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_queue_async_work(
  env: napi_env,
  work: napi_async_work,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, work) }
}
