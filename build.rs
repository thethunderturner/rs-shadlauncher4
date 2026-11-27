fn main() {
    embed_revision();
    embed_windows_icon();
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

#[cfg(windows)]
fn embed_windows_icon() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("src/assets/icon.ico");
    res.compile().expect("Failed to compile Windows resources");
}

#[cfg(not(windows))]
fn embed_windows_icon() {}

fn embed_revision() {
    fn git(args: &[&str]) -> Option<String> {
        let output = std::process::Command::new("git").args(args).output().ok()?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }

    // Follow both detached CI checkouts and local branches, including worktrees.
    for name in ["HEAD", "packed-refs"] {
        if let Some(path) = git(&["rev-parse", "--git-path", name]) {
            if std::path::Path::new(&path).exists() {
                println!("cargo:rerun-if-changed={path}");
            }
        }
    }
    if let Some(reference) = git(&["symbolic-ref", "-q", "HEAD"]) {
        if let Some(path) = git(&["rev-parse", "--git-path", &reference]) {
            println!("cargo:rerun-if-changed={path}");
        }
    }
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
    let revision = git(&["rev-parse", "HEAD"])
        .or_else(|| std::env::var("GITHUB_SHA").ok())
        .unwrap_or_else(|| "unknown".into());
    let date = git(&["show", "-s", "--format=%cI", "HEAD"]).unwrap_or_default();
    println!("cargo:rustc-env=LAUNCHER_COMMIT={revision}");
    println!("cargo:rustc-env=LAUNCHER_COMMIT_DATE={date}");
}
