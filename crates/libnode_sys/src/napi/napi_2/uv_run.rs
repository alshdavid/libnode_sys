use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "uv_run".as_bytes();
type SIGNATURE = fn(loop_: *mut uv_loop_s, mode: uv_run_mode) -> napi_status;
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn uv_run(
  loop_: *mut uv_loop_s,
  mode: uv_run_mode,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(loop_, mode) }
}
