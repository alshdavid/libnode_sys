use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::CString;

pub fn main() -> anyhow::Result<()> {
  let code = r#"console.log('hello world')"#;

  // Find path to libnode
  let lib_path = std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .join("libnode")
    .join("libnode.so");

  // Load libnode
  libnode_sys::load::cdylib(&lib_path)?;

  // Use the path to the current executable as the first argument
  let current_exe = CString::new(std::env::current_exe().unwrap().to_str().unwrap()).unwrap();

  // Args to pass into node.js
  let args = vec!["-e", code];

  // Convert arguments to CStrings
  let args = args
    .iter()
    .map(|arg| CString::new(*arg).unwrap())
    .collect::<Vec<CString>>();
  // Create complete argument list
  let mut final_args = vec![current_exe];
  final_args.extend(args);

  // Convert arguments into pointers
  let c_args = final_args
    .iter()
    .map(|arg| arg.as_ptr())
    .collect::<Vec<*const c_char>>();

  // Start Node.js
  unsafe {
    // Note: Due to a limitation of V8, once started & exited, Nodejs cannot be started again
    libnode_sys::node_embedding_start(c_args.len() as c_int, c_args.as_ptr())
  };

  Ok(())
}
