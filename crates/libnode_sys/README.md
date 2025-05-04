# Libnode Bindings for Rust

This crate exports Rust bindings for Node.js libnode and n-api.

This can be used by:
- Applications embedding `libnode`
- Applications creating n-api extensions

# Installation (embedded)

```bash
# Install library
cargo add libnode_sys

# Download libnode, this must be distributed with your application
# The version below includes a patch with a C FFI for libnode
#
# https://github.com/alshdavid/libnode-prebuilt
#
mkdir -p /opt/libnode
curl \
  -L \
  --url https://github.com/alshdavid/libnode-prebuilt/releases/download/v22.15.0/libnode-linux-amd64.tar.xz \
  | tar -xJvf - -C /opt/libnode
```

# Usage

## Embedded

```rust
fn main() -> anyhow::Result<()> {
  // Load libnode. You must do this before using any other APIs
  libnode_sys::load::cdylib("/opt/libnode/libnode.so")?;

  unsafe {
    // Start Nodejs
    libnode_sys::node_embedding_start(/* ... */);

    Ok(())
  }
}
```

## Embedded injecting native module

```rust
fn main() -> anyhow::Result<()> {
  // Load libnode. You must do this before using any other APIs
  libnode_sys::load::cdylib("/path/to/libnode.so")?;
 
  let nm = Box::into_raw(Box::new(libnode_sys::napi_module {
    // ... register native module
  }))

  unsafe {
    libnode_sys::napi_module_register(nm);

    // Start Nodejs
    libnode_sys::node_embedding_start(/* ... */);

    Ok(())
  }
}
```

# How to include libnode in your application

I recommend distributing the dynamic library alongside your application and finding it at runtime
using a relative path to your executable

```
/my-application
  my-application
  libnode.so
```
