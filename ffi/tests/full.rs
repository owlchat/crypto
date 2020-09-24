use std::{env, path::PathBuf, process::Command};

fn build_full(out_dir_path: PathBuf) {
    let cwd = env::current_dir().unwrap();
    let mut cmd = Command::new("clang");
    #[cfg(not(windows))]
    let lib = "keystore";
    #[cfg(windows)]
    let lib = "keystore.dll";
    let workspace = cwd.parent().unwrap().to_path_buf();
    let lib = format!("-l{}", lib);
    let lib_dir = format!("-L{}", workspace.join("target/debug").display());
    let input = format!("{}", cwd.join("tests/full.c").display());
    let output = format!("-o {}", out_dir_path.join("full").display());
    dbg!(&[&input, &lib, &lib_dir, &output]);
    cmd.args(&[&input, &lib, &lib_dir, &output]);
    assert!(cmd.status().unwrap().success());
}
#[test]
fn test_ffi() {
    let out_path = tempfile::tempdir().unwrap().into_path();
    build_full(out_path.as_path().into());
    let mut cmd = Command::new(out_path.join("full"));
    let out = cmd.output().unwrap();
    let output = String::from_utf8_lossy(&out.stdout);
    println!("{}", output);
    assert!(out.status.success());
    memory_test(out_path);
}

#[cfg(unix)]
fn memory_test(out_dir_path: PathBuf) {
    let mut cmd = Command::new("valgrind");
    cmd.args(&["--error-exitcode=1", "--leak-check=full"])
        .arg(&*out_dir_path.join("full").to_string_lossy());
    let out = cmd.output().unwrap();
    let output = String::from_utf8_lossy(&out.stdout);
    println!("{}", output);
    assert!(out.status.success());
}

#[cfg(windows)]
fn memory_test(out_dir_path: PathBuf) {
    let _ = out_dir_path;
}
