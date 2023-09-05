fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rerun-if-changed={}/../.git/HEAD", crate_dir);
    println!("cargo:rerun-if-changed={}/../.git/index", crate_dir);
}
