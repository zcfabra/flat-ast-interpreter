use crate::lex::{DisplayToken, Kind, Token};

use std::iter::Peekable;

#[derive(Debug)]
pub struct AstRef(usize);

#[derive(Debug)]
pub struct AstPool(Vec<AstExpr>);

impl AstPool {
    pub fn display_ast(&self, src: &str) -> String {
        // TODO: Make efficient
        let root = self.0.last().expect("Tree Was Empty");
        return self.display_expr(root, src);
    }

    fn display_expr(&self, node: &AstExpr, src: &str) -> String {
        match node {
            AstExpr::Literal(token) => DisplayToken::new(src, token).to_string(),
            AstExpr::BinOp(l, op_token, r) => {
                // TODO: Get rid of the .0 stuff
                let op = DisplayToken::new(src, op_token);
                let l_repr = self.display_expr(self.0.get(l.0).expect("Invalid Index"), src);
                let r_repr = self.display_expr(self.0.get(r.0).expect("Invalid Index"), src);

                return format!("( {} {} {} )", op, l_repr, r_repr);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    LOWEST,
    ADDSUB,
    MULDIV,
}

impl AstPool {
    pub fn new() -> Self {
        AstPool(Vec::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, e: AstExpr) -> () {
        self.0.push(e);
    }
}

#[derive(Debug)]
pub enum AstExpr {
    Literal(Token),
    BinOp(AstRef, Token, AstRef),
}

pub fn parse_ast<T>(tokens: T) -> AstPool
where
    T: IntoIterator<Item = Token>,
{
    let mut pool = AstPool::new();

    let mut token_iter = tokens.into_iter().peekable();

    while let Some(expr) = parse_expr(&mut token_iter, &mut pool, Precedence::LOWEST) {
        pool.push(expr);
    }

    return pool;
}

pub fn parse_expr<T>(
    tokens: &mut Peekable<T>,
    pool: &mut AstPool,
    precedence: Precedence,
) -> Option<AstExpr>
where
    T: Iterator<Item = Token>,
{
    let token = tokens.next()?;

    if token.kind == Kind::LParen {
        return parse_expr(tokens, pool, Precedence::LOWEST);
    }

    let mut lhs_expr = AstExpr::Literal(token);

    loop {
        let next_token = match tokens.peek() {
            None => break,
            Some(t) => t,
        };

        if next_token.kind == Kind::RParen {
            tokens.next();
            break;
        }

        if !is_operator(next_token) {
            break;
        }

        let encountered_prec = get_precedence(next_token).expect("Should be operator");

        if encountered_prec < precedence {
            break;
        }

        let op = tokens.next().expect("Should Have Checked Token Already");

        let lhs_ix = pool.len();
        pool.push(lhs_expr);

        let rhs_expr = parse_expr(tokens, pool, encountered_prec)?;
        let rhs_ix = pool.len();
        pool.push(rhs_expr);

        lhs_expr = AstExpr::BinOp(AstRef(lhs_ix), op, AstRef(rhs_ix))
    }

    Some(lhs_expr)
}

pub fn get_precedence(token: &Token) -> Option<Precedence> {
    Some(match token.kind {
        Kind::Add | Kind::Sub => Precedence::ADDSUB,
        Kind::Mul | Kind::Div => Precedence::MULDIV,
        _ => return None,
    })
}
pub fn is_operator(token: &Token) -> bool {
    match token.kind {
        Kind::Add | Kind::Sub | Kind::Mul | Kind::Div => true,
        _ => false,
    }
}
