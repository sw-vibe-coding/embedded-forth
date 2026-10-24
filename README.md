# Embedded Forth

Minimal plain HTML + JS + Rust/WASM prototype for a proto-Forth workbench.

The same engine also runs as a native Linux terminal REPL, with baked-in
Fibonacci, factorial, and FizzBuzz demos. No browser is needed for the CLI.

## Terminal REPL

```bash
cargo build --release --no-default-features
./target/release/embedded-forth
```

Type `MENU`, select `1`, `2`, or `3`, and the selected demo loads and runs.
Its definitions stay in the session: try `10 FIB .` after Fibonacci or
`6 FACT .` after factorial. The demos are baked into the binary, so a board
needs only the executable. Their sources are in `demos/*.fth`.

`MENU` is a terminal command entered on its own line. Other terminal
commands are `:stack`, `:reset`, and `:quit`; Ctrl-D also exits. Forth
definitions can span REPL lines. Use files for multiline comments and
strings, since the REPL evaluates each line separately.

You can also run source files or expressions:

```bash
./target/release/embedded-forth demos/fib.fth
./target/release/embedded-forth -e '2 3 + .'
printf ': SQUARE DUP * ;\n9 SQUARE .\n' | ./target/release/embedded-forth -
```

Forth cells are signed 32-bit integers. Factorial inputs must stay within
0 through 12 to fit. VM diagnostics retain the browser engine's behavior:
they print on stdout; CLI exit status reports argument and host I/O errors.

## Arduino Uno Q

The attached board runs aarch64 Debian Linux and is reached through the
existing SSH tunnel at `arduino@localhost`, port `2222`. The CLI runs on
the Linux processor.

On the build host, install the Rust ARM64 Linux target and provide the
`aarch64-linux-gnu-gcc` cross-compiler, then deploy:

```bash
rustup target add aarch64-unknown-linux-gnu
./scripts/deploy-uno-q.sh
python3 scripts/smoke-uno-q.py
ssh -t -p 2222 arduino@localhost ./embedded-forth/embedded-forth
```

Type `MENU` at the `forth>` prompt. The board needs no Rust installation
and no separate demo files. The executable lives at
`/home/arduino/embedded-forth/embedded-forth`.

Both scripts accept an optional SSH destination and port, for example
`./scripts/deploy-uno-q.sh arduino@board.local 22`. Keep the SSH tunnel
running when using the default destination.

For development, read [AGENTS.md](AGENTS.md) and use `agentrail next` and
`agentrail begin`. Run `scripts/gate.sh` before committing.

The gate runs formatting, clippy, tests, native and WASM builds, spelling,
Markdown validation, and sw-checklist. The inherited engine has ten
checklist size failures. `scripts/checklist-baseline.json` records their
original limits and source commit; the gate prints all failures and rejects
any new or enlarged failure. A passing gate does not mean a clean checklist.

**Live demo:** <https://sw-vibe-coding.github.io/sw-fth-wasm/>

![screenshot](images/screenshot.png?ts=1777240106992)

## What it does

- REPL + Source textareas; live-updated Stack / Dictionary / Output / History / Trace panes
- Rust/WASM `Machine` owns the data stack, return stack, memory, dictionary, tokenizer, compiler, and VM loop
- Colon definitions compile to an opcode IR; user-word execution runs on an iterative VM loop (no host-stack recursion for nested calls)
- Vocabulary covers arithmetic, stack shuffling, comparisons, conditionals (`IF/ELSE/THEN`), post- and pre-test loops (`BEGIN/UNTIL`, `BEGIN/WHILE/REPEAT`), counted loops (`DO/LOOP` with `I`), memory (`VARIABLE`, `CONSTANT`, `@`, `!`, `+!`, `ALLOT`), return stack (`>R R> R@`), I/O (`CR EMIT SPACE`), and introspection (`SEE <word>`, `WORDS`). See [CHANGES.md](CHANGES.md) for the full feature timeline.

## Build

Requirements:

- Rust toolchain with `rustup`
- `wasm-bindgen-cli`

Install `wasm-bindgen-cli` if needed:

```bash
cargo install wasm-bindgen-cli
```

Build:

```bash
./scripts/build.sh
```

Serve the `web/` directory:

```bash
cd web
python3 -m http.server 8000
```

Then open:

```text
http://localhost:8000
```

## Why this shape matters

This is the first clean split between:

- browser shell (HTML + tiny JS)
- runtime engine (`Machine` in Rust/WASM)
- dictionary mapping word names to primitive implementations, user-word op bodies, variables, and constants

The same WASM module can grow more Forth-owned behavior (self-hosting-oriented compiler features, a memory pane, tick + `EXECUTE`, etc.) without changing the browser shell very much.

## Related Repositories

- [sw-cor24-forth](https://github.com/sw-embed/sw-cor24-forth) - sibling DTC Forth targeting the COR24 24-bit RISC ISA
- [sw-cor24-project](https://github.com/sw-embed/sw-cor24-project) - COR24 ecosystem hub

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A. Wright

## License

MIT License. See [LICENSE](LICENSE) for the full text.
