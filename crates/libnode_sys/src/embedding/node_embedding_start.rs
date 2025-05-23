use std::ffi::c_char;
use std::ffi::c_int;
use std::sync::OnceLock;

type SIGNATURE = fn(argc: c_int, argv: *const *const c_char);
static CACHE: OnceLock<crate::load::Symbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn node_embedding_start(
  argc: c_int,
  argv: *const *const c_char,
) {
  CACHE.get_or_init(|| unsafe { crate::load::get_sym(b"node_embedding_start").unwrap() })(
    argc, argv,
  )
}
