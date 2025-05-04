run:
  test -d target/libnode || $(mkdir ./target/libnode && curl -L --url https://github.com/alshdavid/libnode-prebuilt/releases/download/v23.11.0/libnode-linux-amd64.tar.xz | tar -xJvf - -C ./target/libnode)
  cargo run --package libnode_sys_example_basic