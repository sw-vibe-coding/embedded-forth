use proto_forth_wasm::Machine;
use std::io::{self, IsTerminal, Write};

pub(crate) fn repl(machine: &mut Machine) -> Result<(), Box<dyn std::error::Error>> {
    let terminal = io::stdin().is_terminal();
    if terminal {
        println!("embedded-forth — type MENU for demos; Ctrl-D or :quit to exit");
    }
    loop {
        if terminal {
            print!("forth> ");
            io::stdout().flush()?;
        }
        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }
        if line.trim().eq_ignore_ascii_case("MENU") {
            crate::menu::menu(machine)?;
            continue;
        }
        match line.trim() {
            ":quit" => break,
            ":reset" => machine.reset(),
            ":stack" => println!("{}", machine.get_stack_text()),
            _ => machine.eval_repl(&line),
        }
        crate::output(machine)?;
    }
    Ok(())
}
