// https://www.youtube.com/watch?v=pePqWoTnSmQ
// https://rustwiki.org/en/cargo/reference/build-scripts.html

#[cfg(feature = "bindgen")]
fn main() {
    use std::env::var;
    use std::path::PathBuf;
    use bindgen::{Builder, CargoCallbacks};
    use pkg_config::Config;

    Config::new()
        .atleast_version("1.0.18")
        .print_system_libs(false)
        .probe("libsodium")
        .unwrap();

    let bindings = Builder::default()
        .header("soda.h")
        .allowlist_function("sodium_init")
        .allowlist_function("crypto_generichash")
        .allowlist_var("crypto_generichash_.*")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Couldn't generate bindings.");

    // https://rustwiki.org/en/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings.");
}

#[cfg(not(feature = "bindgen"))]
fn main() {}
