use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_delete_reference".as_bytes();
type SIGNATURE = fn(env: napi_env, ref_: napi_ref) -> napi_status;
static CACHE: OnceLock<crate::load::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_delete_reference(
  env: napi_env,
  ref_: napi_ref,
) -> napi_status {
  unsafe { CACHE.get_or_init(|| crate::load::get_sym(SYMBOL).unwrap())(env, ref_) }
}
