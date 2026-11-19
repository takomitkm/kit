// e26kit harness main.rs - compiled beside the real backend.rs.
//
// The kit crate's own test target has pre-existing Unix-only gateway code
// that does not compile on Windows (documented on PR #345), so - exactly as
// the PR's own windows-sync-check harness does - this compiles the branch's
// real src/resilient_fs/backend.rs in isolation and benches the Windows
// Backend::sync_directory implementation, which is where the dir flush and
// the volume-root flush live.
//
// usage: harness <dir> <iters>
#[path = "backend.rs"]
pub mod backend;

use std::path::Path;
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("usage: harness <dir> <iters>");
        std::process::exit(2);
    }
    let dir = a[1].clone();
    let iters: u32 = a[2].trim().parse().unwrap();
    std::fs::create_dir_all(&dir).unwrap();
    let b = backend::DiskBackend;

    // warmup
    for _ in 0..20 {
        let _ = backend::Backend::sync_directory(&b, Path::new(&dir));
    }

    let t0 = Instant::now();
    let mut ok = 0u32;
    let mut err = 0u32;
    let mut first_err = String::new();
    for _ in 0..iters {
        match backend::Backend::sync_directory(&b, Path::new(&dir)) {
            Ok(()) => ok += 1,
            Err(e) => {
                if err == 0 {
                    first_err = e.to_string();
                }
                err += 1;
            }
        }
    }
    let d = t0.elapsed();
    println!(
        "sync_directory: {:.1} ms total | {:.4} ms/call | ok={} err={} first_err={:?} dir={}",
        d.as_secs_f64() * 1000.0,
        d.as_secs_f64() * 1000.0 / iters as f64,
        ok,
        err,
        first_err,
        dir
    );
}