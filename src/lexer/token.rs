use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers and Literals
    IDENTIFIER,
    NUMBER,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LCURLY,
    RCURLY,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ILLEGAL => write!(f, "ILLEGAL"),
            Self::EOF => write!(f, "EOF"),
            Self::IDENTIFIER => write!(f, "IDENTIFIER"),
            Self::NUMBER => write!(f, "NUMBER"),
            Self::ASSIGN => write!(f, "ASSIGN"),
            Self::PLUS => write!(f, "PLUS"),
            Self::COMMA => write!(f, "COMMA"),
            Self::SEMICOLON => write!(f, "SEMICOLON"),
            Self::LPAREN => write!(f, "LPAREN"),
            Self::RPAREN => write!(f, "RPAREN"),
            Self::LCURLY => write!(f, "LCURLY"),
            Self::RCURLY => write!(f, "RCURLY")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_value: String,
    token_type: TokenType
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value: {:>10} Type:{:>20}", self.token_value, self.token_type)
    }
}

impl Token {
    pub fn new(token_value: String, token_type: TokenType) -> Token {
        Token { token_value, token_type }
    }
}

#[test]
fn test_same_token() {
    let token: Token = Token::new(String::from("Hello"), TokenType::IDENTIFIER);
    let expected_token = Token { token_value: String::from("Hello"), token_type: TokenType::IDENTIFIER};
    assert_eq!(token, expected_token);
}

#[test]
fn test_different_token() {
    let token: Token = Token::new(String::from("Hello"), TokenType::IDENTIFIER);
    let expected_token = Token { token_value: String::from("World"), token_type: TokenType::IDENTIFIER};
    assert_ne!(token, expected_token);
}
