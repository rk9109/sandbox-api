use std::io;
use std::process::Output;

use tempfile::TempDir;
use tokio::process::Command;

const MOUNT_PATH: &'static str = "/home/sandbox/mnt/";

// Languages supported
pub enum Language {
    C,
    Cpp,
    Rust,
    Go,
}

// Construct a secure docker command
//
// Limits: (TODO)
// -
// -
// -
// -
pub fn base_command(tmp: &TempDir) -> Command {
    let mut path = tmp.path().as_os_str().to_os_string();
    path.push(":");
    path.push(MOUNT_PATH);

    let mut cmd = Command::new("docker");

    cmd
        .arg("run")
        .arg("--rm")
        .arg("--volume").arg(&path);

    cmd
}

pub fn compile_c_command(tmp: &TempDir) -> Command {
    let mut cmd = base_command(tmp);

    cmd
        .arg("sandbox-c")
        .arg("clang")
        .arg("-o")
        .arg("mnt/output")
        .arg("mnt/input.c");

    cmd
}

pub fn compile_cpp_command(tmp: &TempDir) -> Command {
    let mut cmd = base_command(tmp);

    cmd
        .arg("sandbox-cpp")
        .arg("clang++")
        .arg("-o")
        .arg("mnt/output")
        .arg("mnt/input.cpp");

    cmd
}

pub fn compile_rust_command(tmp: &TempDir) -> Command {
    let mut cmd = base_command(tmp);

    cmd
        .arg("sandbox-rust")
        .arg("rustc")
        .arg("-o")
        .arg("mnt/output")
        .arg("mnt/input.rs");

    cmd
}

pub fn compile_go_command(tmp: &TempDir) -> Command {
    let mut cmd = base_command(tmp);

    cmd
        .arg("sandbox-go")
        .arg("go")
        .arg("build")
        .arg("-o")
        .arg("mnt/output")
        .arg("mnt/input.go");

    cmd
}

pub fn execute_command(tmp: &TempDir) -> Command {
    let mut cmd = base_command(tmp);

    // reuse C sandbox to execute binaries
    cmd
        .arg("sandbox-c")
        .arg("mnt/output");

    cmd
}

#[tokio::main]
pub async fn run_command(mut cmd: Command) -> Result<Output, io::Error> {
    cmd.output().await
}
