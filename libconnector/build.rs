use cbindgen::Config;
use regex::Regex;
use std::env;
use std::io::Write;

fn main() {
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = env::var("OUT_DIR").unwrap();
    let target_dir = format!("{}/../../../../", target_dir);
    let config = Config::from_file(format!("{}/cbindgen.toml", crate_dir)).unwrap();

    let build_info = common_build_info::info::load();
    let git_version = build_info.git_hash.as_str();

    let regex = Regex::new(r"v([\d]+)\.([\d]+)\.([\d]+)-([\s\S]+)").unwrap();
    let (major, minor, patch, build) = if let Some(capture) = regex.captures(git_version) {
        let major = capture.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let minor = capture.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let patch = capture.get(3).unwrap().as_str().parse::<u8>().unwrap();
        let build = capture.get(4).unwrap().as_str();
        (major, minor, patch, build)
    } else {
        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap();
        (major, minor, patch, git_version)
    };

    let path = format!("{}/{}", env::var("OUT_DIR").unwrap(), "/cbuilt.rs");
    let mut file = std::fs::File::create(path).unwrap();

    writeln!(file, "const VERSION_MAJOR: u8 = {};", major).unwrap();
    writeln!(file, "const VERSION_MINOR: u8 = {};", minor).unwrap();
    writeln!(file, "const VERSION_PATCH: u8 = {};", patch).unwrap();
    writeln!(
        file,
        "const VERSION_BUILD: *const std::os::raw::c_char = \"{}, {}, {}, {}, {}\\0\".as_ptr() as *const _;",
        build, build_info.ci_platform, build_info.profile, build_info.time, build_info.compiler,
    )
    .unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}/include/{}.h", target_dir, crate_name));

    std::fs::copy(
        format!("{}/res/connector_helper.h", crate_dir),
        format!("{}/include/connector_helper.h", target_dir),
    )
    .unwrap();

    println!("cargo:rerun-if-changed={}/../.git/HEAD", crate_dir);
    println!("cargo:rerun-if-changed={}/../.git/index", crate_dir);
}
