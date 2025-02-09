use std::process::Command;

#[test]
fn test_hello_world_output() {
    let output = Command::new("./target/release/smallest_hello_world")
        .output()
        .expect("Failed to execute binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "Hello, world!");
}