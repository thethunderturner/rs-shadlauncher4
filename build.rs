fn main() {
    let directory = "externals/libatrac9/C/src";
    println!("cargo:rerun-if-changed={directory}");
    let mut files: Vec<_> = std::fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "c"))
        .collect();
    files.sort();
    cc::Build::new()
        .files(files)
        .include(directory)
        .define("_USE_MATH_DEFINES", None)
        .warnings(false)
        .compile("atrac9");
    if std::env::var("CARGO_CFG_TARGET_FAMILY").as_deref() == Ok("unix") {
        println!("cargo:rustc-link-lib=m");
    }
}
