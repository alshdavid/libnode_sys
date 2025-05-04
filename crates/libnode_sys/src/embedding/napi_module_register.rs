use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

#[allow(non_camel_case_types)]
pub type napi_addon_register_func =
  Option<unsafe extern "C" fn(env: napi_env, exports: napi_value) -> napi_value>;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct napi_module {
  pub nm_version: c_int,
  pub nm_flags: c_uint,
  pub nm_filename: *const c_char,
  pub nm_register_func: napi_addon_register_func,
  pub nm_modname: *const c_char,
  pub nm_priv: *mut c_void,
  pub reserved: [*mut c_void; 4usize],
}

static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();
const SYMBOL: &[u8] = "napi_module_register".as_bytes();

type SIGNATURE = fn(mod_: *mut napi_module) -> napi_status;

pub unsafe fn napi_module_register(mod_: *mut napi_module) -> napi_status {
  CACHE.get_or_init(|| unsafe { crate::load::get_sym(SYMBOL).unwrap() })(mod_)
}
