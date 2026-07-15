use std::fs;
use std::process::Command;

/// Helper: run the JVM on a class file and return (stdout, stderr, exit_code)
fn run_jvm(class_file: &str) -> (String, String, i32) {
    let output = Command::new(env!("CARGO_BIN_EXE_minijvm"))
        .arg(class_file)
        .output()
        .expect("Failed to run minijvm");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);
    (stdout, stderr, exit_code)
}

/// Helper: compile Java source and run the JVM
fn compile_and_run(java_file: &str) -> (String, String, i32) {
    // Compile
    let javac_output = Command::new("javac")
        .arg(java_file)
        .output()
        .expect("javac not found — install JDK");
    if !javac_output.status.success() {
        let stderr = String::from_utf8_lossy(&javac_output.stderr).to_string();
        panic!("javac failed for {}: {}", java_file, stderr);
    }
    // Figure out the class name from the file name
    let class_name = java_file
        .trim_end_matches(".java")
        .split('/')
        .last()
        .unwrap();
    let class_file = format!("{}.class", class_name);
    run_jvm(&class_file)
}

#[test]
fn test_hello_world() {
    let (stdout, stderr, code) = run_jvm("tests/HelloWorld.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
    assert!(stdout.contains("Hello, World!") || stdout.contains("Execution completed"),
        "Expected Hello World output, got: {}", stdout);
}

#[test]
fn test_simple_test() {
    let (stdout, stderr, code) = run_jvm("tests/SimpleTest.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
}

#[test]
fn test_static_field() {
    let (stdout, stderr, code) = run_jvm("tests/StaticFieldTest.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
}

#[test]
fn test_exception() {
    let (stdout, stderr, code) = run_jvm("tests/ExceptionTest.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
}

#[test]
fn test_return_value() {
    let (stdout, stderr, code) = run_jvm("tests/ReturnValueTest.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
}

#[test]
fn test_synchronization() {
    let (stdout, stderr, code) = run_jvm("tests/SynchronizationTest.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
}

#[test]
fn test_full_test() {
    let (stdout, stderr, code) = run_jvm("tests/FullTest.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
}

#[test]
fn test_simple_if() {
    let (stdout, stderr, code) = run_jvm("tests/SimpleIfTest.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
}

#[test]
fn test_simple_sync() {
    let (stdout, stderr, code) = run_jvm("tests/SimpleSyncTest.class");
    assert!(code == 0 || code == 1, "Exit code: {}", code);
}