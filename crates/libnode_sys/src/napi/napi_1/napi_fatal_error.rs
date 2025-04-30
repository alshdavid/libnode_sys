use std::ffi::c_char;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_fatal_error".as_bytes();
type SIGNATURE = fn(
  location: *const c_char,
  location_len: isize,
  message: *const c_char,
  message_len: isize,
) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_fatal_error(
  location: *const c_char,
  location_len: isize,
  message: *const c_char,
  message_len: isize,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      location,
      location_len,
      message,
      message_len,
    )
  }
}
