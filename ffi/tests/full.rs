use std::{env, process::Command};

fn build_full() {
    let cwd = env::current_dir().unwrap();
    let workspace = cwd.parent().unwrap().to_path_buf();
    let mut cmd = Command::new("clang");
    let out = workspace.join("target").join("debug");
    env::set_current_dir(out.as_path()).unwrap();
    #[cfg(not(windows))]
    let lib = format!("{}", out.join("keystore.so").display()); // only linux for now
    #[cfg(windows)]
    let lib = format!("-l{}", out.join("keystore.dll").display());
    #[cfg(windows)]
    let exe = "full.exe";
    #[cfg(not(windows))]
    let exe = "full";
    let input = format!("{}", cwd.join("tests/full.c").display());
    let output = format!("-o{}", exe);
    cmd.args(&[&output, &lib, &input]);
    assert!(cmd.status().unwrap().success());
}
#[test]
fn test_ffi() {
    build_full();
    let mut cmd = Command::new("full");
    let out = cmd.output().unwrap();
    let output = String::from_utf8_lossy(&out.stdout);
    println!("{}", output);
    assert!(out.status.success());
    memory_test();
}

#[cfg(unix)]
fn memory_test() {
    let mut cmd = Command::new("valgrind");
    cmd.args(&[
        "--error-exitcode=1",
        "--leak-check=full",
        "--track-origins=yes",
        "./full",
    ]);
    let out = cmd.output().unwrap();
    let output = String::from_utf8_lossy(&out.stdout);
    println!("{}", output);
    assert!(out.status.success());
}

#[cfg(windows)]
fn memory_test() {}
