#[derive(Debug, Clone)]
pub enum TokenType {
    Return,
    Int,
    Semi,
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
    index: usize,
}

impl Tokeniser {
    pub fn new(src: String) -> Self {
        Tokeniser {
            src,
            tokens: None,
            index: 0
        }
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

    fn peek(&self, ahead: usize) -> Option<Vec<Token>> {
        if self.index + ahead >= self.src.len() {
            return None; //potentially add some error message here. Not sure how this would work yet
        }
        let mut ret: Vec<Token> = Vec::new();
        for i in self.index + 1..self.index + ahead + 1 {
            ret.push(self.tokens.clone().unwrap()[i].clone());
        }

        Some(ret)
    }

    fn consume(&mut self, ahead: usize) {
        self.index += ahead;
    }
}
