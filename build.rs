// build.rs
// Minimal build script for linking custom linker scripts if needed.

fn main() {
    println!("cargo:rerun-if-changed=link.x");
}
