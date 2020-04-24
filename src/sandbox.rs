use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::process::Output;
use std::result;
use std::string;

use tempfile::TempDir;

use crate::command::*;

#[derive(Debug)]
pub enum SandboxError {
    StringConversionError(string::FromUtf8Error),
    TempError(io::Error),
    PermissionsError(io::Error),
    SourceFileError(io::Error),
    CompilationError(io::Error),
    CompilationOutputError,
    ExecutionError(io::Error),
}

pub type Result<T> = result::Result<T, SandboxError>;

pub struct CommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    fn new(output: Output) -> Result<Self> {
        let success = output.status.success();
        let stdout = String::from_utf8(output.stdout).map_err(SandboxError::StringConversionError)?;
        let stderr = String::from_utf8(output.stderr).map_err(SandboxError::StringConversionError)?;

        Ok(CommandOutput {
            success,
            stdout,
            stderr,
        })
    }
}

pub struct SandboxOutput {
    pub compile_output: CommandOutput,
    pub execute_output: CommandOutput,
}

pub struct Sandbox {
    tmp: TempDir,
    language: Language,
}

impl Sandbox {
    /// TODO document
    ///
    ///
    pub fn new(code: &str, language: Language) -> Result<Self> {
        // create a tempdir to mount onto the running container; needs 
        // rwx permissions so the container can read/write/execute files
        // in the tempdir
        let tmp = TempDir::new().map_err(SandboxError::TempError)?;
        fs::set_permissions(tmp.path(), PermissionsExt::from_mode(0o777))
            .map_err(SandboxError::PermissionsError)?;

        // create input file containing source code in tempdir
        let extension = match language {
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Rust => "rs",
            Language::Go => "go",
        };
        let mut path = tmp.path().join("input");
        path.set_extension(extension);
        fs::write(&path, code).map_err(SandboxError::SourceFileError)?;

        Ok(Sandbox { tmp, language })
    }

    /// TODO document
    ///
    ///
    pub fn output(&self) -> Result<SandboxOutput> {
        // run compilation
        let compile_output = CommandOutput::new(self.compile()?)?;

        // if compilation is not successful, return intermediate output
        // from compiler
        if !compile_output.success {
            return Ok(SandboxOutput {
                compile_output,
                execute_output: CommandOutput {
                    success: false,
                    stdout: String::from(""),
                    stderr: String::from(""),
                },
            });
        }

        // run execution
        let execute_output = CommandOutput::new(self.execute()?)?;

        Ok(SandboxOutput {
            compile_output,
            execute_output,
        })
    }

    fn compile(&self) -> Result<Output> {
        // construct compilation command
        let cmd = match self.language {
            Language::C => compile_c_command(&self.tmp),
            Language::Cpp => compile_cpp_command(&self.tmp),
            Language::Rust => compile_rust_command(&self.tmp),
            Language::Go => compile_go_command(&self.tmp),
        };

        // run compilation command
        let output = run_command(cmd).map_err(SandboxError::CompilationError)?;

        if output.status.success() {
            // successful compilation creates an executable `output` in the
            // bind mount
            let output_path = self.tmp.path().join("output");
            if !output_path.exists() {
                return Err(SandboxError::CompilationOutputError);
            }
        }

        Ok(output)
    }

    fn execute(&self) -> Result<Output> {
        // construct execution command
        let cmd = execute_command(&self.tmp);

        // run execution command
        let output = run_command(cmd).map_err(SandboxError::ExecutionError)?;

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compile_c_code() {
        let code = r#"
        #include <stdio.h>

        int main() {
            printf("C test");
            return 0;
        }
        "#;

        let c_output = Sandbox::new(code, Language::C)
            .expect("Failed to construct sandbox")
            .output()
            .expect("Failed to execute");

        assert_eq!(c_output.execute_output.stdout, "C test");
    }

    #[test]
    fn compile_cpp_code() {
        let code = r#"
        #include <iostream>

        int main() {
            std::cout << "C++ test";
            return 0;
        }
        "#;

        let cpp_output = Sandbox::new(code, Language::Cpp)
            .expect("Failed to construct sandbox")
            .output()
            .expect("Failed to execute");

        assert_eq!(cpp_output.execute_output.stdout, "C++ test");
    }

    #[test]
    fn compile_rust_code() {
        let code = r#"
        fn main() {
            print!("Rust test");
        }
        "#;

        let rust_output = Sandbox::new(code, Language::Rust)
            .expect("Failed to construct sandbox")
            .output()
            .expect("Failed to execute");

        assert_eq!(rust_output.execute_output.stdout, "Rust test");
    }

    #[test]
    fn compile_go_code() {
        let code = r#"
        package main

        import "fmt"

        func main() {
            fmt.Print("Go test")
        }
        "#;

        let go_output = Sandbox::new(code, Language::Go)
            .expect("Failed to construct sandbox")
            .output()
            .expect("Failed to execute");

        assert_eq!(go_output.execute_output.stdout, "Go test");
    }

    #[test]
    fn pid_limit() {
        let code = r#"
        #include <stdio.h>
        #include <unistd.h>

        int main() {
            int pid;
            for (int i = 0; i < 256; i++) {
                pid = fork();
                if (pid == -1) {
                    printf("cannot fork");
                }
                if (pid == 0) {
                    return 0;
                }
            }
            return 0;
        }
        "#;

        let output = Sandbox::new(code, Language::C)
            .expect("Failed to construct sandbox")
            .output()
            .expect("Failed to execute");

        assert!(output.execute_output.stdout.contains("cannot fork"));
    }

    #[test]
    fn net_limit() {
        let code = r#"
        package main

        import "fmt"
        import "net/http"

        func main() {
            _, err := http.Get("http://example.com/")
            if err != nil {
                fmt.Print("cannot connect")
            }
        }
        "#;

        let output = Sandbox::new(code, Language::Go)
            .expect("Failed to construct sandbox")
            .output()
            .expect("Failed to execute");

        assert!(output.execute_output.stdout.contains("cannot connect"));
    }
}
