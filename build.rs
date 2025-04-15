use std::env;
use std::path::PathBuf;

const POSSIBLE_INCLUDE_DIR: [&'static str; 2] = ["/usr/include", "/usr/local/include"];
const POSSIBLE_LIB_DIR: [&'static str; 2] = ["/usr/lib", "/usr/local/lib"];

fn find_include_dir() -> Option<&'static str> {
    for include_dir in POSSIBLE_INCLUDE_DIR {
        let header_dir = PathBuf::from(include_dir.to_owned());
        let mut header_file = header_dir.clone();
        header_file.push("gpib");
        header_file.push("ib.h");
        if header_file.exists() {
            return Some(include_dir);
        }
    }
    None
}

fn find_library_dir() -> Option<&'static str> {
    for library_dir in POSSIBLE_LIB_DIR {
        let library_dir_path = PathBuf::from(library_dir.to_owned());
        let mut library_file = library_dir_path.clone();
        library_file.push("libgpib.so");
        if library_file.exists() {
            return Some(library_dir);
        }
    }
    None
}

fn linux_gpib() {
    let _include_dir = find_include_dir().expect("gpib/ib.h not found.");
    let lib_dir = find_library_dir().expect("libgpib.so not found.");
    println!(r"cargo:rustc-link-search={lib_dir}");
    println!(r"cargo:rustc-link-lib=dylib=gpib");

    let bindings = bindgen::Builder::default()
        .header("headers/gpib.h")
        .clang_arg(r"-I{include_dir}")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for gpib/ib.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("linux_gpib.rs"))
        .expect("Couldn't write linux_gpib.rs");
}

fn main() {
    linux_gpib();
}
