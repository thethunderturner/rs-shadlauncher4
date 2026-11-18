fn main() {
    let source = "externals/libatrac9/C/src";
    println!("cargo:rerun-if-changed={source}");
    let mut build = cc::Build::new();
    build.include(source).warnings(false);
    for entry in std::fs::read_dir(source).expect("bundled libatrac9 source missing") {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|extension| extension == "c") {
            build.file(path);
        }
    }
    build.compile("atrac9");
    if std::env::var("CARGO_CFG_UNIX").is_ok() {
        println!("cargo:rustc-link-lib=m");
    }
}
