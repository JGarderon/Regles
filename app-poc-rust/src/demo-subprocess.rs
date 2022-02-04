use std::io::Write;
use std::process::{Command, Stdio};

fn main() { 
    let mut child = Command::new("python3")
        .arg( "./support.py" ) 
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(
            r#"{"ok":"ok"}"#.as_bytes()
        ).expect("Failed to write to stdin (1)");
        stdin.write_all(
            "\n".as_bytes()
        ).expect("Failed to write to stdin (1)");
        stdin.write_all(
            r#"{"ko":"ko"}"#.as_bytes()
        ).expect("Failed to write to stdin (2)");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    println!("{:?}", output); 
} 