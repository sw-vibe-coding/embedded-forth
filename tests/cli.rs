use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn piped_source_preserves_multiline_comments_and_definitions() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_embedded-forth"))
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(
        b"( a comment\nacross lines )\n: squares 5 0 DO I DUP * . LOOP ;\nsquares\nVARIABLE x 42 x ! x @ .\n",
    ).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success());
    assert_eq!(
        String::from_utf8(result.stdout).unwrap(),
        "defined SQUARES (8 tokens)\nVARIABLE X at addr 1\n0 1 4 9 16 42 \n"
    );
}

#[test]
fn eval_arguments_share_dictionary_without_repeating_output() {
    let result = Command::new(env!("CARGO_BIN_EXE_embedded-forth"))
        .args(["-e", ": sq DUP * ; 6 sq .", "-e", "7 sq ."])
        .output()
        .unwrap();
    assert!(result.status.success());
    assert_eq!(
        String::from_utf8(result.stdout).unwrap(),
        "defined SQ (2 tokens)\n36 \n49 \n"
    );
}

#[test]
fn missing_file_fails() {
    let result = Command::new(env!("CARGO_BIN_EXE_embedded-forth"))
        .arg("/nonexistent/embedded-forth-test.fth")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(!result.stderr.is_empty());
}

#[test]
fn menu_loads_each_demo_and_leaves_words_available() {
    for (choice, check, expected) in [
        ("1", "10 FIB .", "55 "),
        ("2", "6 FACT .", "720 "),
        ("3", "FIZZBUZZ", "FizzBuzz"),
    ] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_embedded-forth"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        write!(
            child.stdin.take().unwrap(),
            "MENU\n{choice}\n{check}\n:quit\n"
        )
        .unwrap();
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success());
        let text = String::from_utf8(result.stdout).unwrap();
        assert!(text.contains(expected), "{text}");
        if choice != "3" {
            assert!(text.ends_with(&format!("{expected}\n")), "{text}");
        } else {
            assert_eq!(text.lines().filter(|line| *line == "FizzBuzz").count(), 12);
        }
        assert!(!text.contains("unknown"), "{text}");
        assert!(!text.contains("empty"), "{text}");
    }
}

#[test]
fn help_and_version_have_short_and_detailed_forms() {
    let call = |arg| {
        let output = Command::new(env!("CARGO_BIN_EXE_embedded-forth"))
            .arg(arg)
            .output()
            .unwrap();
        assert!(output.status.success());
        String::from_utf8(output.stdout).unwrap()
    };
    assert!(call("--help").len() > call("-h").len());
    let version = call("--version");
    assert!(version.len() > call("-V").len());
    for field in [
        "Copyright",
        "License",
        "Repository",
        "Build host",
        "Build commit",
        "Build time",
    ] {
        assert!(version.contains(field), "{version}");
    }
}
