use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rustc-link-lib=pgport");
    println!("cargo:rustc-link-lib=pgcommon");
    println!("cargo:rustc-link-lib=pq");

    println!("cargo:rustc-link-search=../postgres/src/port");
    println!("cargo:rustc-link-search=../postgres/src/common");
    println!("cargo:rustc-link-search=../postgres/src/interfaces/libpq");

    bindgen::Builder::default()
        .header("./src/libpq-in.h")
        .clang_arg("-I../postgres/src/interfaces/libpq/")
        .clang_arg("-I../postgres/src/include/")
        .newtype_enum("ConnStatusType")
        .newtype_enum("ExecStatusType")
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
