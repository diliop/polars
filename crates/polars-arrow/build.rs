fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let channel = version_check::Channel::read().unwrap();
    if channel.is_nightly() {
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    }
    if version_check::is_min_version("1.76.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=simd_prelude");
    }
}
