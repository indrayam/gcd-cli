use assert_cmd::Command;
use predicates::prelude::*;

fn gcd() -> Command {
    Command::cargo_bin("gcd").unwrap()
}

#[test]
fn basic() {
    gcd()
        .args(["48", "18"])
        .assert()
        .success()
        .stdout("6\n")
        .stderr("");
}

#[test]
fn coprime() {
    gcd().args(["17", "5"]).assert().success().stdout("1\n");
}

#[test]
fn one_zero() {
    gcd().args(["0", "5"]).assert().success().stdout("5\n");
}

#[test]
fn both_zero() {
    gcd().args(["0", "0"]).assert().success().stdout("0\n");
}

#[test]
fn negative() {
    gcd()
        .args(["--", "-48", "18"])
        .assert()
        .success()
        .stdout("6\n");
}

#[test]
fn i64_min_with_zero() {
    gcd()
        .args(["--", "-9223372036854775808", "0"])
        .assert()
        .success()
        .stdout("9223372036854775808\n");
}

#[test]
fn missing_arg() {
    gcd().args(["5"]).assert().failure().code(2).stdout("");
}

#[test]
fn not_a_number() {
    gcd().args(["5", "x"]).assert().failure().code(2).stdout("");
}

#[test]
fn help() {
    gcd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}
