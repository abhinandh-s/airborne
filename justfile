check: 
  cargo build 
  cargo clippy --all 
  cargo doc --no-deps
  cargo fmt --all -v

fmt:
  cargo fmt --all -v
