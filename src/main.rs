use std::io;

mod ast;
mod utils;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub cacau_parser);

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("Error: {err}.");
        std::process::exit(1);
    });
}

fn run() -> io::Result<()> {
    let input = utils::read_input()?;

    run_cacau(input);

    Ok(())
}

fn run_cacau(text: String) {
    let ast = calculator::ExprParser::new().parse(&text);
    let _ = dbg!(ast);
}

#[test]
fn calculator() {
    assert!(calculator::ExprParser::new().parse("22").is_ok());
    assert!(calculator::ExprParser::new().parse("(22)").is_ok());
    assert!(calculator::ExprParser::new().parse("((((22))))").is_ok());
    assert!(calculator::ExprParser::new().parse("((((22))))").is_ok());
    assert!(calculator::ExprParser::new()
        .parse("((((22) + 2)))")
        .is_ok());
    assert!(calculator::ExprParser::new().parse("((22)").is_err());
    assert!(calculator::ExprParser::new().parse("((a2)").is_err());
}
