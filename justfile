check: 
  cargo build 
  cargo clippy --all 
  cargo doc --no-deps
  cargo fmt --all -v

fmt:
  cargo fmt --all -v

coverage:
  cargo tarpaulin --exclude-files src/compute.rs src/dataset.rs src/error.rs 

coverage_in_txt:
  cargo tarpaulin --exclude-files src/compute.rs src/dataset.rs src/error.rs --out stdout > tarpaulin_out.txt
