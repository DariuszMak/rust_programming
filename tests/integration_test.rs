#[test]
fn test_hello_world() {
    let output = std::process::Command::new("cargo")
        .arg("run")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, world!"));
}
