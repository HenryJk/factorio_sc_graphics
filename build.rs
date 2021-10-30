extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("external/CascLib")
        .define("CASC_BUILD_STATIC_LIB", "ON")
        .profile("Release")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=casc");
    println!("cargo:rustc-link-lib=dylib=stdc++");                    // stdc++
    println!("cargo:rustc-link-lib=dylib=z");                         // zlib
}
