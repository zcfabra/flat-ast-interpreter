mod lex;

use lex::{DisplayToken, Lexer};

struct AstRef(i32);
struct AstPool(Vec<AstRef>);

impl AstPool {
    pub fn new(src: &str) -> Self {
        AstPool(Vec::new())
    }
}

fn main() {
    let src = "10 + (203 * 10) + aasdasd";
    let lexer = Lexer::new(src);

    for t in lexer.into_iter().map(|t| DisplayToken::new(src, t)) {
        print!("{}", t);
    }
    print!("\n");
}
