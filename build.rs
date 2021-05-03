extern crate cmake;
// extern crate bindgen;
use cmake::Config;

fn main() {
    let dst = Config::new("CascLib")
        .define("CASC_BUILD_STATIC_LIB", "ON")
        .profile("Release")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=casc");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // let bindings = bindgen::Builder::default()
    //     .header(
    //         dst.join("include")
    //             .join("CascLib.h")
    //             .into_os_string()
    //             .into_string()
    //             .unwrap()
    //     )
    //     .blocklist_item("IPPORT_RESERVED")
    //     .generate()
    //     .expect("Unable to generate bindings");
    // bindings
    //     .write_to_file(dst.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
    // bindings
    //     .write_to_file("reference.rs").unwrap();
}