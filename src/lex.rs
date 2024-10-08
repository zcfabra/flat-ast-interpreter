#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Add,
    Sub,
    Mul,
    Div,

    Num,
    Ident,

    LParen,
    RParen,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            match self {
                Self::Add => "ADD",
                Self::Sub => "SUB",
                Self::Mul => "MUL",
                Self::Div => "DIV",

                Self::LParen => "LPAREN",
                Self::RParen => "RPAREN",

                _ => "TOKEN",
            }
        )
    }
}

#[derive(Debug)]
enum Started {
    Numeric,
    Ident,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    start: usize,
    length: usize,
}

impl Token {
    fn new(kind: Kind, start: usize, length: usize) -> Self {
        Token {
            kind,
            start,
            length,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {} -> {})", self.kind, self.start, self.length)
    }
}

#[derive(Clone)]
pub struct Lexer<'src> {
    pub ix: usize,
    pub src: &'src str,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Lexer { ix: 0, src }
    }

    fn skip_whitespace(&mut self) -> () {
        while let Some(' ') = self.src.chars().nth(self.ix) {
            self.ix += 1;
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let start = self.ix;
        let rest = &self.src[start..];

        let c = &rest.chars().next()?;
        self.ix += c.len_utf8();

        let started = match c {
            '+' => return Some(Token::new(Kind::Add, start, 1)),
            '-' => return Some(Token::new(Kind::Sub, start, 1)),
            '*' => return Some(Token::new(Kind::Mul, start, 1)),
            '/' => return Some(Token::new(Kind::Div, start, 1)),

            '(' => return Some(Token::new(Kind::LParen, start, 1)),
            ')' => return Some(Token::new(Kind::RParen, start, 1)),

            '0'..='9' => Started::Numeric,
            c if c.is_alphabetic() => Started::Ident,
            x => {
                println!("Encountered Unknown `{}`", x);
                todo!()
            }
        };

        match started {
            Started::Ident => {
                let end = &rest
                    .find(|c: char| !c.is_alphabetic())
                    .unwrap_or(self.src.len());

                let tok_size = end - start;
                self.ix += tok_size - c.len_utf8();

                return Some(Token::new(Kind::Ident, start, tok_size));
            }
            Started::Numeric => {
                let tok_len = &rest.find(|c| !matches!(c, '0'..='9')).unwrap_or(rest.len());

                self.ix += tok_len - c.len_utf8();

                return Some(Token::new(Kind::Num, start, *tok_len));
            }
        }
    }
}

#[derive(Debug)]
pub struct DisplayToken<'src, 'tok> {
    pub src: &'src str,
    pub token: &'tok Token,
}

impl<'src, 'tok> DisplayToken<'src, 'tok> {
    pub fn new(src: &'src str, token: &'tok Token) -> Self {
        DisplayToken { src, token }
    }
}

impl<'src, 'tok> std::fmt::Display for DisplayToken<'src, 'tok> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.token.start;
        let end = start + self.token.length;

        let tok_str = &self.src[start..end];
        write!(f, "{}", tok_str)
    }
}

#[test]
fn test_basic_lexing() {
    let cases = vec![
        (
            "++--**//",
            vec![
                Token::new(Kind::Add, 0, 1),
                Token::new(Kind::Add, 1, 1),
                Token::new(Kind::Sub, 2, 1),
                Token::new(Kind::Sub, 3, 1),
                Token::new(Kind::Mul, 4, 1),
                Token::new(Kind::Mul, 5, 1),
                Token::new(Kind::Div, 6, 1),
                Token::new(Kind::Div, 7, 1),
            ],
        ),
        (
            "abcabc*123123",
            vec![
                Token::new(Kind::Ident, 0, 6),
                Token::new(Kind::Mul, 6, 1),
                Token::new(Kind::Num, 7, 6),
            ],
        ),
        (
            "10*20-30+40/50",
            vec![
                Token::new(Kind::Num, 0, 2),
                Token::new(Kind::Mul, 2, 1),
                Token::new(Kind::Num, 3, 2),
                Token::new(Kind::Sub, 5, 1),
                Token::new(Kind::Num, 6, 2),
                Token::new(Kind::Add, 8, 1),
                Token::new(Kind::Num, 9, 2),
                Token::new(Kind::Div, 11, 1),
                Token::new(Kind::Num, 12, 2),
            ],
        ),
    ];

    for (input, output) in cases {
        let found = Lexer::new(input).collect::<Vec<Token>>();
        assert_eq!(found, output);
    }
}

#[test]
fn test_whitespace() {
    let results = Lexer::new("          10          +          10          ")
        .into_iter()
        .collect::<Vec<Token>>();
    assert_eq!(
        results,
        vec![
            Token::new(Kind::Num, 10, 2),
            Token::new(Kind::Add, 22, 1),
            Token::new(Kind::Num, 33, 2),
        ]
    )
}

#[test]
fn test_parens() {
    let result = Lexer::new("(((((())))))")
        .into_iter()
        .collect::<Vec<Token>>();

    assert_eq!(
        result,
        vec![
            Token::new(Kind::LParen, 0, 1),
            Token::new(Kind::LParen, 1, 1),
            Token::new(Kind::LParen, 2, 1),
            Token::new(Kind::LParen, 3, 1),
            Token::new(Kind::LParen, 4, 1),
            Token::new(Kind::LParen, 5, 1),
            Token::new(Kind::RParen, 6, 1),
            Token::new(Kind::RParen, 7, 1),
            Token::new(Kind::RParen, 8, 1),
            Token::new(Kind::RParen, 9, 1),
            Token::new(Kind::RParen, 10, 1),
            Token::new(Kind::RParen, 11, 1),
        ]
    )
}
