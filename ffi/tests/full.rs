use std::{env, process::Command};

fn build_full() {
    let cwd = env::current_dir().unwrap();
    let workspace = cwd.parent().unwrap().to_path_buf();
    let mut cmd = Command::new("clang");
    #[cfg(not(windows))]
    let lib = "keystore";
    #[cfg(windows)]
    let lib = "keystore.dll";
    #[cfg(windows)]
    let exe = "full.exe";
    #[cfg(not(windows))]
    let exe = "full";
    let lib = format!("-l{}", lib);
    let lib_dir = format!("-L{}", workspace.join("target/debug").display());
    let input = format!("{}", cwd.join("tests/full.c").display());
    let out = workspace.join("target").join("debug");
    env::set_current_dir(out.as_path()).unwrap();
    let output = format!("-o{}", exe);
    cmd.args(&[&input, &lib, &lib_dir, &output]);
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
    cmd.args(&["--error-exitcode=1", "--leak-check=full", "full"]);
    let out = cmd.output().unwrap();
    let output = String::from_utf8_lossy(&out.stdout);
    println!("{}", output);
    assert!(out.status.success());
}

#[cfg(windows)]
fn memory_test() {}
