use assert_cmd::Command;

fn gcd() -> Command {
    Command::cargo_bin("gcd").unwrap()
}

#[test]
fn twelve_and_eighteen_prints_six() {
    gcd().args(["12", "18"]).assert().success().stdout("6\n");
}

#[test]
fn negative_after_dash_dash_prints_six() {
    gcd()
        .args(["--", "-48", "18"])
        .assert()
        .success()
        .stdout("6\n");
}

#[test]
fn help_includes_example_section() {
    gcd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Examples:"))
        .stdout(predicates::str::contains("gcd 12 18"))
        .stdout(predicates::str::contains("gcd -- -48 18"));
}

#[test]
fn missing_arg_exits_two() {
    gcd().args(["5"]).assert().failure().code(2);
}

#[test]
fn non_integer_exits_two() {
    gcd().args(["5", "x"]).assert().failure().code(2);
}
