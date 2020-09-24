use std::{path::PathBuf, process::Command};

fn build_full() {
    let mut cmd = Command::new("clang");
    let out = PathBuf::from("../target/debug");
    #[cfg(not(windows))]
    let lib = format!("{}", out.join("keystore.so").display()); // only linux for now
    #[cfg(windows)]
    let lib = format!("-l{}", out.join("keystore.dll").display());
    #[cfg(windows)]
    let exe = out.join("full.exe");
    #[cfg(not(windows))]
    let exe = out.join("full");
    let input = "./tests/full.c".to_string();
    let output = format!("-o{}", exe.display());
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
