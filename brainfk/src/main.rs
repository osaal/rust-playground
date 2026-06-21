use libbrainfk::interpreter::Interpreter;
use std::env::args;

fn main() {
    let mut args = args();
    // 1st arg is useless for us
    let _ = args.next();

    if let Some(input) = args.next() {
        let interpreter = Interpreter::new();
        match interpreter.input(input).run() {
            Ok(_) => println!("Execution finished."),
            Err(e) => println!("{e}"),
        }
    }
}
