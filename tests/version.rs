use proto_forth_wasm::Machine;

#[test]
fn version_is_a_forth_word_with_no_stack_effect() {
    let mut machine = Machine::new();
    machine.take_output();
    machine.eval_repl("42 version");
    let output = machine.take_output();
    for field in [
        "embedded-forth 0.1.0",
        "Build host:",
        "Build commit:",
        "Build time:",
    ] {
        assert!(output.contains(field), "{output}");
    }
    assert_eq!(machine.get_stack_text(), "42");
    assert!(
        machine
            .get_dictionary_text()
            .lines()
            .any(|word| word == "VERSION")
    );
}

#[test]
fn version_compiles_executes_and_survives_saved_state() {
    let mut machine = Machine::new();
    machine.take_output();
    machine.eval_repl(": ABOUT VERSION ; ABOUT");
    let output = machine.take_output();
    assert!(output.contains("Build commit:"), "{output}");
    let saved = machine.save_state();
    let mut restored = Machine::new();
    assert!(restored.load_state(&saved));
    restored.take_output();
    restored.eval_repl("' VERSION EXECUTE ABOUT");
    let output = restored.take_output();
    assert_eq!(output.matches("Build commit:").count(), 2, "{output}");
}

#[test]
fn version_matches_cli_metadata_and_preserves_output_order() {
    let run = |args: &[&str]| {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_embedded-forth"))
            .args(args)
            .output()
            .unwrap();
        assert!(output.status.success());
        String::from_utf8(output.stdout).unwrap()
    };
    let word = run(&["-e", "VERSION"]);
    assert_eq!(word.lines().count(), 4);
    assert!(run(&["--version"]).starts_with(&word));
    assert_eq!(run(&["-e", "1 . VERSION 2 ."]), format!("1 \n{word}2 \n"));
}

#[test]
fn previous_schema_is_rejected_cleanly() {
    let mut machine = Machine::new();
    let mut state: serde_json::Value = serde_json::from_str(&machine.save_state()).unwrap();
    state["version"] = 3.into();
    assert!(!machine.load_state(&state.to_string()));
    assert!(machine.take_output().contains("State version mismatch"));
}
