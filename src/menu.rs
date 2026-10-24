use proto_forth_wasm::Machine;
use std::io::{self, Write};

const DEMOS: [(&str, &str); 3] = [
    ("fib.fth", include_str!("../demos/fib.fth")),
    ("fact.fth", include_str!("../demos/fact.fth")),
    ("fizzbuzz.fth", include_str!("../demos/fizzbuzz.fth")),
];

pub(crate) fn menu(machine: &mut Machine) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "1) fib.fth — Fibonacci\n2) fact.fth — factorial\n3) fizzbuzz.fth — FizzBuzz\n0) return to REPL"
    );
    print!("demo> ");
    io::stdout().flush()?;
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let index = match choice.trim().to_ascii_lowercase().as_str() {
        "1" | "fib" | "fib.fth" => 0,
        "2" | "fact" | "fact.fth" => 1,
        "3" | "fizzbuzz" | "fizzbuzz.fth" => 2,
        "" | "0" => return Ok(()),
        _ => {
            println!("Unknown demo. Type MENU to try again.");
            return Ok(());
        }
    };
    let (name, source) = DEMOS[index];
    println!("Loading {name}");
    machine.eval_repl(source);
    crate::output(machine)?;
    Ok(())
}
