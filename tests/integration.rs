use assert_cmd::Command;
use rstest::rstest;

const TEXT_1: &str = "some text\nanother\nsome\n";
const TEXT_2: &str = "some  text   \n   another\n  some\n ";

const SINGLE_LINE_1: &str = "sometext some\n";
const SINGLE_LINE_2: &str = "sometextsome\n";
const SINGLE_LINE_3: &str = "sometext  some  \n";

const SLICE_OF_LINE_1: &str = "some text hello";
const SLICE_OF_LINE_2: &str = " sometext hello ";

const WORD_1: &str = "hello";
const WORD_2: &str = "hello1.all";

const SNAKE_CASE_1: &str = "some_word";
const SNAKE_CASE_2: &str = "some_word1_all";

const KEBAB_CASE_1: &str = "some-word";
const KEBAB_CASE_2: &str = "some-word1-all";

const CAMEL_CASE_1: &str = "someWord";
const CAMEL_CASE_2: &str = "someWord1All";

const PASCAL_CASE_1: &str = "SomeWord";
const PASCAL_CASE_2: &str = "SomeWord1All";

fn cmd() -> Command {
    Command::cargo_bin("txc").unwrap()
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
#[case::text_1(TEXT_1, "some text another some\n")]
#[case::text_1(TEXT_2, "some text another some\n")]
#[case::single_line_1(SINGLE_LINE_1, SINGLE_LINE_1)]
#[case::single_line_2(SINGLE_LINE_2, SINGLE_LINE_2)]
#[case::single_line_3(SINGLE_LINE_3, SINGLE_LINE_3)]
#[case::slice_of_line_1(SLICE_OF_LINE_1, SLICE_OF_LINE_1)]
#[case::slice_of_line_2(SLICE_OF_LINE_2, SLICE_OF_LINE_2)]
#[case::word_1(WORD_1, WORD_1)]
#[case::word_2(WORD_2, WORD_2)]
#[case::snake_case_1(SNAKE_CASE_1, SNAKE_CASE_1)]
#[case::snake_case_2(SNAKE_CASE_2, SNAKE_CASE_2)]
#[case::kebab_case_1(KEBAB_CASE_1, KEBAB_CASE_1)]
#[case::kebab_case_2(KEBAB_CASE_2, KEBAB_CASE_2)]
#[case::camel_case_1(CAMEL_CASE_1, CAMEL_CASE_1)]
#[case::camel_case_2(CAMEL_CASE_2, CAMEL_CASE_2)]
#[case::pascal_case_1(PASCAL_CASE_1, PASCAL_CASE_1)]
#[case::pascal_case_2(PASCAL_CASE_2, PASCAL_CASE_2)]
fn test_to_single_line(#[case] stdin: &str, #[case] stdout: &str) {
    cmd()
        .arg("--to")
        .arg("single-line")
        .write_stdin(stdin)
        .assert()
        .stdout(stdout.to_string());
}

#[rstest]
#[case::text_1(TEXT_1, TEXT_1)]
#[case::text_2(TEXT_2, TEXT_2)]
#[case::single_line_1(SINGLE_LINE_1, SINGLE_LINE_1)]
#[case::single_line_2(SINGLE_LINE_2, SINGLE_LINE_2)]
#[case::single_line_3(SINGLE_LINE_3, SINGLE_LINE_3)]
#[case::slice_of_line_1(SLICE_OF_LINE_1, SLICE_OF_LINE_1)]
#[case::slice_of_line_2(SLICE_OF_LINE_2, SLICE_OF_LINE_2)]
#[case::word_1(WORD_1, WORD_1)]
#[case::word_2(WORD_2, WORD_2)]
#[case::snake_case_1(SNAKE_CASE_1, SNAKE_CASE_1)]
#[case::snake_case_2(SNAKE_CASE_2, SNAKE_CASE_2)]
#[case::kebab_case_1(KEBAB_CASE_1, "some_word")]
#[case::kebab_case_2(KEBAB_CASE_2, "some_word1_all")]
#[case::camel_case_1(CAMEL_CASE_1, "some_word")]
#[case::camel_case_2(CAMEL_CASE_2, "some_word1_all")]
#[case::pascal_case_1(PASCAL_CASE_1, "some_word")]
#[case::pascal_case_2(PASCAL_CASE_2, "some_word1_all")]
fn test_to_snake_case(#[case] stdin: &str, #[case] stdout: &str) {
    cmd()
        .arg("--to")
        .arg("snake-case")
        .write_stdin(stdin)
        .assert()
        .stdout(stdout.to_string());
}

#[rstest]
#[case::text_1(TEXT_1, TEXT_1)]
#[case::text_2(TEXT_2, TEXT_2)]
#[case::single_line_1(SINGLE_LINE_1, SINGLE_LINE_1)]
#[case::single_line_2(SINGLE_LINE_2, SINGLE_LINE_2)]
#[case::single_line_3(SINGLE_LINE_3, SINGLE_LINE_3)]
#[case::slice_of_line_1(SLICE_OF_LINE_1, SLICE_OF_LINE_1)]
#[case::slice_of_line_2(SLICE_OF_LINE_2, SLICE_OF_LINE_2)]
#[case::word_1(WORD_1, WORD_1)]
#[case::word_2(WORD_2, WORD_2)]
#[case::snake_case_1(SNAKE_CASE_1, "some-word")]
#[case::snake_case_2(SNAKE_CASE_2, "some-word1-all")]
#[case::kebab_case_1(KEBAB_CASE_1, KEBAB_CASE_1)]
#[case::kebab_case_2(KEBAB_CASE_2, KEBAB_CASE_2)]
#[case::camel_case_1(CAMEL_CASE_1, "some-word")]
#[case::camel_case_2(CAMEL_CASE_2, "some-word1-all")]
#[case::pascal_case_1(PASCAL_CASE_1, "some-word")]
#[case::pascal_case_2(PASCAL_CASE_2, "some-word1-all")]
fn test_to_kebab_case(#[case] stdin: &str, #[case] stdout: &str) {
    cmd()
        .arg("--to")
        .arg("kebab-case")
        .write_stdin(stdin)
        .assert()
        .stdout(stdout.to_string());
}

#[rstest]
#[case::text_1(TEXT_1, TEXT_1)]
#[case::text_2(TEXT_2, TEXT_2)]
#[case::single_line_1(SINGLE_LINE_1, SINGLE_LINE_1)]
#[case::single_line_2(SINGLE_LINE_2, SINGLE_LINE_2)]
#[case::single_line_3(SINGLE_LINE_3, SINGLE_LINE_3)]
#[case::slice_of_line_1(SLICE_OF_LINE_1, SLICE_OF_LINE_1)]
#[case::slice_of_line_2(SLICE_OF_LINE_2, SLICE_OF_LINE_2)]
#[case::word_1(WORD_1, WORD_1)]
#[case::word_2(WORD_2, WORD_2)]
#[case::snake_case_1(SNAKE_CASE_1, "someWord")]
#[case::snake_case_2(SNAKE_CASE_2, "someWord1All")]
#[case::kebab_case_1(KEBAB_CASE_1, "someWord")]
#[case::kebab_case_2(KEBAB_CASE_2, "someWord1All")]
#[case::camel_case_1(CAMEL_CASE_1, CAMEL_CASE_1)]
#[case::camel_case_2(CAMEL_CASE_2, CAMEL_CASE_2)]
#[case::pascal_case_1(PASCAL_CASE_1, "someWord")]
#[case::pascal_case_2(PASCAL_CASE_2, "someWord1All")]
fn test_to_camel_case(#[case] stdin: &str, #[case] stdout: &str) {
    cmd()
        .arg("--to")
        .arg("camel-case")
        .write_stdin(stdin)
        .assert()
        .stdout(stdout.to_string());
}

#[rstest]
#[case::text_1(TEXT_1, TEXT_1)]
#[case::text_2(TEXT_2, TEXT_2)]
#[case::single_line_1(SINGLE_LINE_1, SINGLE_LINE_1)]
#[case::single_line_2(SINGLE_LINE_2, SINGLE_LINE_2)]
#[case::single_line_3(SINGLE_LINE_3, SINGLE_LINE_3)]
#[case::slice_of_line_1(SLICE_OF_LINE_1, SLICE_OF_LINE_1)]
#[case::slice_of_line_2(SLICE_OF_LINE_2, SLICE_OF_LINE_2)]
#[case::word_1(WORD_1, WORD_1)]
#[case::word_2(WORD_2, WORD_2)]
#[case::snake_case_1(SNAKE_CASE_1, "SomeWord")]
#[case::snake_case_2(SNAKE_CASE_2, "SomeWord1All")]
#[case::kebab_case_1(KEBAB_CASE_1, "SomeWord")]
#[case::kebab_case_2(KEBAB_CASE_2, "SomeWord1All")]
#[case::camel_case_1(CAMEL_CASE_1, "SomeWord")]
#[case::camel_case_2(CAMEL_CASE_2, "SomeWord1All")]
#[case::pascal_case_1(PASCAL_CASE_1, PASCAL_CASE_1)]
#[case::pascal_case_2(PASCAL_CASE_2, PASCAL_CASE_2)]
fn test_to_pascal_case(#[case] stdin: &str, #[case] stdout: &str) {
    cmd()
        .arg("--to")
        .arg("pascal-case")
        .write_stdin(stdin)
        .assert()
        .stdout(stdout.to_string());
}

// #[rstest]
// fn test_to_word(#[case] stdin: String, #[case] stdout: String) {
//     cmd()
//         .arg("--to")
//         .arg("word")
//         .write_stdin(stdin)
//         .assert()
//         .stdout(stdout);
// }
