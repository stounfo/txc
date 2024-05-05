use assert_cmd::Command;
use rstest::rstest;

fn cmd() -> Command {
    Command::cargo_bin("txc").unwrap()
}

#[rstest]
fn test_runs_okay() {
    cmd().assert().success();
}

#[rstest]
fn test_help() {
    cmd().arg("--help").assert().success();
    cmd().arg("-h").assert().success();
}

#[rstest]
fn test_version() {
    cmd().arg("--version").assert().success();
    cmd().arg("-V").assert().success();
}

#[rstest]
#[case::several_lines(
    "42 43 44   \n44 46 47  \n     48 49 50\n",
    "42 43 44 44 46 47 48 49 50\n"
)]
#[case::line("42 43 44\n", "42 43 44\n")]
#[case::slice("42 43 44", "42 43 44\n")]
#[case::several_spaces("42  43", "42 43\n")]
fn test_to_one_line(#[case] stdin: String, #[case] stdout: String) {
    cmd()
        .arg("--format-to")
        .arg("one-line")
        .write_stdin(stdin)
        .assert()
        .stdout(stdout);
}
