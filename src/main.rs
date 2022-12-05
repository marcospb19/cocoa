mod utils;

use std::io;

fn run() -> io::Result<()> {
    let input = utils::read_input().unwrap_or_else(|err| panic!("Could not read input: {err}"));

    let ast = cocoa::parse_input(&input).unwrap();
    cocoa::interpret_ast(&ast).unwrap();

    Ok(())
}

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("Error: {err}.");
        std::process::exit(1);
    });
}
