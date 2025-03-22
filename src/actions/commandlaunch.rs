use std::process::Command;

use crate::loader::util::{SherlockError, SherlockErrorType};

pub fn command_launch(exec: &str, keyword: &str) -> Result<(), SherlockError> {
    let exec = exec.replace("{keyword}", &keyword);
    let commands: Vec<&str> = exec.split("&").collect();
    let num_cmds = commands.len();

    for command in commands {
        if !command.is_empty() {
            let mut parts = command.split_whitespace();
            let execute = parts.next().expect("No command found");
            let args: Vec<&str> = parts.collect();

            let _output = if num_cmds > 1 {
                // Asynchronous execution (output)
                asynchronous_execution(execute, args)?;
            } else {
                // Synchronous execution (output)
                synchronous_execution(execute, args)?;
            };
        }
    }
    Ok(())
}

fn synchronous_execution(execute: &str, args: Vec<&str>) -> Result<String, SherlockError> {
    let output = Command::new(execute)
        .args(&args)
        .output()
        .map_err(|e| SherlockError {
            error: SherlockErrorType::CommandExecutionError(format!("{}", execute)),
            traceback: e.to_string(),
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(SherlockError {
            error: SherlockErrorType::CommandExecutionError(format!("{}", execute)),
            traceback: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}
fn asynchronous_execution(execute: &str, args: Vec<&str>) -> Result<String, SherlockError> {
    let async_command = Command::new(execute)
        .args(&args)
        .spawn()
        .map_err(|e| SherlockError {
            error: SherlockErrorType::CommandExecutionError(format!("{}", execute)),
            traceback: e.to_string(),
        })?;

    Ok(format!("{:?}", async_command))
}
