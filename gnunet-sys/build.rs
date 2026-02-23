use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_inline_functions(true)
        .ctypes_prefix("libc")
        .raw_line("#[allow(non_upper_case_globals)]")
        .raw_line("#[allow(non_camel_case_types)]")
        .raw_line("#[allow(non_snake_case)]")
        .raw_line("#[allow(dead_code)]")
        .raw_line("#[allow(deref_nullptr)]")
        .raw_line("#[allow(unused_imports)]")
        .raw_line("")
        .raw_line("use libc::*;")
        .clang_arg("-I/usr/include/gnunet")
        .clang_arg("-I/usr/include")
        .blocklist_type("max_align_t")
        .allowlist_type("GNUNET_.*")
        .allowlist_function("GNUNET_.*")
        .allowlist_var("GNUNET_.*");

    println!("cargo:rustc-link-lib=gnunetutil");
    println!("cargo:rustc-link-lib=gnunetcadet");
    println!("cargo:rustc-link-lib=gnunetidentity");
    println!("cargo:rustc-link-lib=gnunetgns");
    println!("cargo:rustc-link-lib=gnunetarm");
    println!("cargo:rustc-link-lib=gnunetpeerstore");
    println!("cargo:rustc-link-lib=gnunetmessenger");

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
