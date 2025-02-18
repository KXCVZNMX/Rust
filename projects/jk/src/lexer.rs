use crate::lexer::ArithmeticOps::Sub;

#[derive(Debug, Clone)]
pub enum ArithmeticOps {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Return,
    Int,
    Semi,
    AOps(ArithmeticOps),
    Var,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub val: Option<String>,
}

#[derive(Debug)]
pub struct Tokeniser {
    src: String,
    tokens: Option<Vec<Token>>,
}

impl Tokeniser {
    pub fn new(src: String) -> Self {
        Tokeniser {
            src,
            tokens: None,
        }
    }

    fn is_valid_token(token: &str) -> Option<Token> {
        const VALID_TOKEN: Vec<(&str, TokenType)> = vec![
            ("return", TokenType::Return),
            (";", TokenType::Semi),
            ("+", TokenType::AOps(ArithmeticOps::Add)),
            ("-", TokenType::AOps(ArithmeticOps::Sub)),
            ("*", TokenType::AOps(ArithmeticOps::Mul)),
            ("/", TokenType::AOps(ArithmeticOps::Div)),
        ];
    }

    pub fn tokenise(&mut self) {
        let tokens: Vec<_> = self.src.split_whitespace().collect();
        let mut ret: Vec<Token> = Vec::new();
        for s in tokens {
            let val: Token = match s {
                "return" => Token { token: TokenType::Return, val: None},
                ";" => Token { token: TokenType::Semi, val: None},
                _ if s.parse::<i128>().is_ok() => Token{ token: TokenType::Int, val: Some(s.to_string())},
                _ => panic!("invalid token type: {}", s),
            };
            ret.push(val);
        }

        self.tokens = Some(ret);
    }
}
