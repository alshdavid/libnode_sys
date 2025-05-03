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
  unsafe {
    // Load libnode. You must do this before using any other APIs
    libnode_sys::load::cdylib("/opt/libnode/libnode.so")?;

    // Note, this is not yet available in mainline
    // https://github.com/nodejs/node/pull/54660
    libnode_sys::node_embedding_main_run();

    Ok(())
  }
}
```

## Embedded injecting native module

```rust
fn main() -> anyhow::Result<()> {
  unsafe {
    // Load libnode. You must do this before using any other APIs
    libnode_sys::load::cdylib("/path/to/libnode.so")?;

    libnode::napi_module_register(Box::into_raw(Box::new(libnode_sys::napi_module {
      // ... register native module
    })))

    // Note, this is not yet available in mainline
    // https://github.com/nodejs/node/pull/54660
    libnode_sys::node_embedding_main_run();

    Ok(())
  }
}
```
