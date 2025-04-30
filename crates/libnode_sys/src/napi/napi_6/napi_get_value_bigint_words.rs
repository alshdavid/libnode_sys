use std::ffi::c_int;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_value_bigint_words".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  value: napi_value,
  sign_bit: *mut c_int,
  word_count: *mut usize,
  words: *mut u64,
) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_value_bigint_words(
  env: napi_env,
  value: napi_value,
  sign_bit: *mut c_int,
  word_count: *mut usize,
  words: *mut u64,
) -> napi_status {
  unsafe {
    CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(
      env, value, sign_bit, word_count, words,
    )
  }
}
