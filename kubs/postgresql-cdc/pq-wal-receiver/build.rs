fn add_objects(mut cb: impl FnMut(std::path::PathBuf) -> (), paths: &[impl AsRef<std::path::Path>]) {
    for out_path in paths {
        let dir = std::fs::read_dir(out_path).unwrap();
        for dir_entry in dir {
            let e = dir_entry.unwrap();
            let ft = e.file_type().unwrap();

            if ft.is_file() {
                let path = e.path();
                let is_o = path.as_path().extension().map(|os| os.to_string_lossy()).map(|str| str.eq("o")).unwrap_or(false);
                if is_o {
                    cb(path);
                }
            }
        }
    }
}

fn read_dirs(paths: &[impl AsRef<std::path::Path>]) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();
    for path in paths {
        result.push(path.as_ref().into());
        for dir_entry in std::fs::read_dir(path).unwrap() {
            let entry = dir_entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                result.push(entry.path());
            }
        }
    }
    result
}

fn main() {
    // Built with pushd ./postgres/src/common && make
    println!("cargo:rustc-link-lib=pgcommon_srv");
    println!("cargo:rustc-link-search=../postgres/src/common");
    // Built with pushd ./postgres/src/port && make
    println!("cargo:rustc-link-lib=pgport_srv");
    println!("cargo:rustc-link-search=../postgres/src/port");
    // Built with pushd ./postgres/src/interfaces/libpq && make
    println!("cargo:rustc-link-lib=pq");
    println!("cargo:rustc-link-search=../postgres/src/interfaces/libpq");
    // ICU
    println!("cargo:rustc-link-lib=icui18n");
    println!("cargo:rustc-link-lib=icuuc");
    println!("cargo:rustc-link-lib=icudata");
    // Z_LIB
    println!("cargo:rustc-link-lib=z");

    let mut build = cc::Build::new();
    let dirs_rec = read_dirs(&[
        "../postgres/src/backend/access/",
        "../postgres/src/backend/archive/",
        "../postgres/src/backend/backup/",
        "../postgres/src/backend/bootstrap/",
        "../postgres/src/backend/catalog/",
        "../postgres/src/backend/parser/",
        "../postgres/src/backend/commands/",
        "../postgres/src/backend/executor/",
        "../postgres/src/backend/foreign/",
        "../postgres/src/backend/lib/",
        "../postgres/src/backend/libpq/",
        "../postgres/src/backend/main/",
        "../postgres/src/backend/nodes/",
        "../postgres/src/backend/optimizer/",
        "../postgres/src/backend/partitioning/",
        "../postgres/src/backend/port/",
        "../postgres/src/backend/postmaster/",
        "../postgres/src/backend/regex/",
        "../postgres/src/backend/replication/",
        "../postgres/src/backend/rewrite/",
        "../postgres/src/backend/statistics/",
        "../postgres/src/backend/storage/",
        "../postgres/src/backend/tcop/",
        "../postgres/src/backend/tsearch/",
        "../postgres/src/backend/utils/",
        "../postgres/src/backend/jit/",
        "../postgres/src/timezone/",
    ]);
    for dir in dirs_rec {
        let bref = &mut build;
        add_objects(move |p| {
            bref.object(p);
        }, &[dir]);
    }

    build
        .file("./progname.c")   // Define progname from backend/main/main.c
        .file("../postgres/src/backend/replication/libpqwalreceiver/libpqwalreceiver.c")
        .include("../postgres/src/include/libpq")
        .include("../postgres/src/include/mb")
        .include("../postgres/src/include/")
        .include("../postgres/src/interfaces/libpq/")
        .compile("libpqwalreceiver");

    let bindings = bindgen::Builder::default()
        .header("../postgres/src/backend/replication/libpqwalreceiver/libpqwalreceiver.c")
        .clang_arg("-I../postgres/src/include/")
        .clang_arg("-I../postgres/src/interfaces/libpq/")
        .allowlist_file("../postgres/src/backend/replication/libpqwalreceiver/libpqwalreceiver.c")
        .generate()
        .unwrap();
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
