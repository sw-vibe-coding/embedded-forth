use proto_forth_wasm::Machine;
mod info;
mod menu;
mod session;

use std::io::{self, Read, Write};

fn output(machine: &mut Machine) -> io::Result<()> {
    let text = machine.take_output();
    if !text.is_empty() {
        writeln!(io::stdout(), "{text}")?;
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut machine = Machine::new();
    machine.take_output(); // The browser's startup transcript is not CLI output.
    let mut args = std::env::args().skip(1);
    let mut batch = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                info::help(arg == "--help");
                return Ok(());
            }
            "-V" | "--version" => {
                info::version(arg == "--version");
                return Ok(());
            }
            "-e" | "--eval" => {
                let src = args.next().ok_or("missing source after --eval")?;
                machine.eval_repl(&src);
            }
            "-" => {
                let mut src = String::new();
                io::stdin().read_to_string(&mut src)?;
                machine.eval_repl(&src);
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option: {arg}").into()),
            _ => machine.eval_repl(&std::fs::read_to_string(arg)?),
        }
        batch = true;
        output(&mut machine)?;
    }
    if batch {
        return Ok(());
    }
    session::repl(&mut machine)
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("embedded-forth: {err}");
            std::process::ExitCode::FAILURE
        }
    }
}
