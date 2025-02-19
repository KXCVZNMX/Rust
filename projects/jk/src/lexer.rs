#[derive(Debug, Clone)]
pub enum ArithmeticOps {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Datatype {
    Int,
    Float,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Return,
    Int,
    Float,
    Semi,
    AOps(ArithmeticOps),
    Var,
    Assignment,
    TypeId(Datatype),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub val: Option<String>,
}

#[derive(Debug)]
pub struct Tokens {
    src: String,
    tokens: Option<Vec<Token>>,
}

impl Tokens {
    pub fn new(src: String) -> Self {
        Tokens {
            src,
            tokens: None,
        }
    }

    fn is_valid_token(token: &str) -> Option<Token> {
        match token {
            "return" => Some(Token {val: Some("return".to_string()), token: TokenType::Return}),
            ";" => Some(Token {val: None, token: TokenType::Semi}),
            "+" => Some(Token {val: None, token: TokenType::AOps(ArithmeticOps::Add)}),
            "-" => Some(Token {val: None, token: TokenType::AOps(ArithmeticOps::Sub)}),
            "*" => Some(Token {val: None, token: TokenType::AOps(ArithmeticOps::Mul)}),
            "/" => Some(Token {val: None, token: TokenType::AOps(ArithmeticOps::Div)}),
            ":=" => Some(Token {val: None, token: TokenType::Assignment}),
            "int" => Some(Token {val: None, token: TokenType::TypeId(Datatype::Int)}),
            "float" => Some(Token {val: None, token: TokenType::TypeId(Datatype::Float)}),
            _ if token.parse::<i128>().is_ok() => Some(Token { token: TokenType::Int, val: Some(token.to_string())}),
            _ if token.parse::<f64>().is_ok() => Some(Token { token: TokenType::Float, val: Some(token.to_string())}),
            _ => Some(Token {val: Some(token.to_string()), token: TokenType::Var}),
        }
    }

    pub fn tokenise(&mut self) {
        let tokens: Vec<_> = self.src.split_whitespace().collect();
        let mut ret: Vec<Token> = Vec::new();
        for s in tokens {
            let val: Token = Self::is_valid_token(s).unwrap();
            ret.push(val);
        }

        self.tokens = Some(ret);
    }
}
