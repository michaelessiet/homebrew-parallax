use std::env;
use std::process::Stdio;
use tokio;
use colored::*;
use std::time::Instant;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command as AsyncCommand;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", "Usage: parallel \"command1\" \"command2\" ...".red());
        std::process::exit(1);
    }

    let commands = &args[1..];
    let start_time = Instant::now();

    println!("\n{}", format!("Running {} commands", commands.len()).blue());
    println!("");

    let handles: Vec<_> = commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| {
            let cmd = cmd.clone();
            tokio::spawn(async move {
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                if parts.is_empty() {
                    return (i, format!("Empty command provided at position {}", i), false);
                }

                let program = parts[0];
                let args = &parts[1..];

                println!("{}", format!("[{}]: Starting command: {}", i, cmd).yellow());

                let mut child = AsyncCommand::new(program)
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("failed to spawn command");

                let stdout = child.stdout.take()
                    .expect("child did not have a handle to stdout");

                let stderr = child.stderr.take()
                    .expect("child did not have a handle to stderr");

                let mut stdout_reader = BufReader::new(stdout).lines();
                let mut stderr_reader = BufReader::new(stderr).lines();

                let stdout_handle = tokio::spawn(async move {
                    while let Ok(Some(line)) = stdout_reader.next_line().await {
                        println!("[{}]: {}", i, line);
                    }
                });

                let stderr_handle = tokio::spawn(async move {
                    while let Ok(Some(line)) = stderr_reader.next_line().await {
                        println!("[{}]: {}", i, line.red());
                    }
                });

                let status = child.wait().await.unwrap();
                stdout_handle.await.unwrap();
                stderr_handle.await.unwrap();

                (i, cmd, status.success())
            })
        })
        .collect();

    println!("");

    for handle in handles {
        let (i, cmd, success) = handle.await.unwrap();
        let status = if success { "✓".green() } else { "✗".red() };
        println!("{} Command {}: {}", status, i + 1, cmd);
    }

    println!("\n{}", format!("All commands completed in {:.2}ms", start_time.elapsed().as_secs_f64() * 1000.0).green());
}
