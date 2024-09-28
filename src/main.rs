mod lex;
mod parse;

use lex::{DisplayToken, Lexer};
use parse::parse_ast;

fn main() {
    let src = "10 * (20 + 30) * 20";
    let lexer = Lexer::new(src);

    println!("{}", parse_ast(lexer).display_ast(src));
}
