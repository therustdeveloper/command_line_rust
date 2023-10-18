use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("command_line_rust").unwrap();
    cmd.assert().success();
}

#[test]
fn output() {
    let mut cmd = Command::cargo_bin("command_line_rust").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}