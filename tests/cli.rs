use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn works() {
    let mut cmd = Command::cargo_bin("command-line-rust").unwrap();
    cmd.assert().success().stdout("xmchx\n");
}
