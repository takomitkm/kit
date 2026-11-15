//! Windows-only harness for the `resilient_fs::backend` module.
//!
//! The real crate's test target has pre-existing Unix-only gateways that do not
//! compile on Windows, so this compiles the exact backend source in isolation
//! and runs its `cfg(all(test, windows))` tests on a Windows runner.
//!
//! The workflow copies `src/resilient_fs/backend.rs` next to this file before
//! running `cargo test`, so the compiled source is the branch's real file.

pub mod backend;
pub use backend::*;

fn main() {}
