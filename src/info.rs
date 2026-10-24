pub(crate) fn help(long: bool) {
    println!(
        "Usage: embedded-forth [-e SOURCE] [FILE ...]\nType MENU in the REPL for demos. Ctrl-D or :quit exits."
    );
    if long {
        println!(
            "\nWith no arguments, start a line-oriented REPL.\n\
            -e, --eval SOURCE   Evaluate Forth source\n\
            FILE               Evaluate a file\n\
            -                  Evaluate all stdin as one source\n\
            -h, --help         Show help\n\
            -V, --version      Show version\n\n\
            REPL commands: MENU, :quit, :reset, :stack.\n\
            MENU loads baked-in fib.fth, fact.fth, or fizzbuzz.fth.\n\
            Cells are signed 32-bit. VM diagnostics go to stdout;\n\
            exit status reports host I/O and argument errors.\n\n\
            AI CODING AGENT INSTRUCTIONS:\n\
            Read AGENTS.md and run agentrail next/begin before working.\n\
            Run scripts/gate.sh; commit before agentrail complete."
        );
    }
}

pub(crate) fn version(long: bool) {
    if long {
        println!("{}", proto_forth_wasm::build_version());
        println!(
            "Copyright (c) 2026 Michael A. Wright\nLicense: MIT\n\
            Repository: https://github.com/sw-vibe-coding/embedded-forth"
        );
    } else {
        println!("embedded-forth {}", env!("CARGO_PKG_VERSION"));
    }
}
