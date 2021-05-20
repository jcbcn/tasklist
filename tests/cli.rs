use tempdir::TempDir;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn database_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tl")?;
    let tmp_dir = TempDir::new("integration")?;

    cmd.current_dir(tmp_dir.path());

    cmd.arg("tasks").arg("get");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unable to open database file"));

    tmp_dir.close()?;
    Ok(())
}