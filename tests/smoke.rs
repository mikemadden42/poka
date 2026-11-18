use std::process::Command;

#[test]
fn prints_goodbye() {
    let output = Command::new(env!("CARGO_BIN_EXE_goodbye"))
        .output()
        .expect("failed to run goodbye");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Goodbye, World! This is a native Rust program.\n"
    );
}
