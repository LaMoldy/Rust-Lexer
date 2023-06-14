use super::token::{Token, TokenType};

#[derive(PartialEq, Debug)]
pub struct Lexer {
    character: u8,
    cursor: usize,
    input: Vec<u8>
}

impl Lexer {
    pub fn new(file_content: String) -> Lexer {
        Lexer { cursor: 0, character: 0, input: file_content.into_bytes() }
    }

    fn remove_whitespace(&mut self) {
        while self.character.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn get_identifier(&mut self) -> String {
        let mut result = String::new();

        while self.character.is_ascii_alphabetic() || self.character == b'_' {
            result = result + char::from(self.character).to_string().as_str();
            self.read_char();
        }

        result
    }

    fn get_number(&mut self) -> String {
        let mut result = String::new();

        while self.character.is_ascii_digit() {
            result = result + char::from(self.character).to_string().as_str();
            self.read_char();
        }

        result
    }

    fn read_char(&mut self) {
        if self.cursor >= self.input.len() {
            self.character = 0;
        }
        else {
            self.character = self.input[self.cursor];
        }
        
        self.cursor += 1;
    }

    fn next_token(&mut self) -> Token {
        self.remove_whitespace();

        let mut token_value = String::new();

        let token_type = match self.character {
            b'a'..=b'z' | b'A'..=b'Z' => {
                token_value = self.get_identifier();
                self.cursor -= 1;
                TokenType::IDENTIFIER
            },
            b'0'..=b'9' => {
                token_value = self.get_number();
                self.cursor -= 1;
                TokenType::NUMBER
            },
            b'{' => TokenType::LCURLY,
            b'}' => TokenType::RCURLY,
            b'(' => TokenType::LPAREN,
            b')' => TokenType::RPAREN,
            b'=' => TokenType::ASSIGN,
            b'+' => TokenType::PLUS,
            b';' => TokenType::SEMICOLON,
            b':' => TokenType::COLON,
            b',' => TokenType::COMMA,
            b'"' => TokenType::DQUOTES,
            0 => TokenType::EOF,
            _ => TokenType::ILLEGAL

        };

        if token_value == String::from("") {
            token_value = char::from(self.character).to_string();
        }

        self.read_char();
        Token::new(token_value, token_type)
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.cursor <= self.input.len() {
            let token: Token = self.next_token();
            tokens.push(token);
        }

        tokens
    }
}

#[test]
fn same_lexer_object() {
    let content_example = String::from("Testing");
    let lex = Lexer::new(content_example.clone());
    let expected_lex = Lexer { cursor: 0, character: 0, input: content_example.into_bytes() };

    assert_eq!(lex, expected_lex);
}

#[test]
fn different_lexer_object() {
    let content_example = String::from("Testing");
    let different_content_example = String::from("Test");
    let lex = Lexer::new(content_example);
    let expected_lex = Lexer { cursor: 0, character: 0, input: different_content_example.into_bytes() };

    assert_ne!(lex, expected_lex);
}

#[test]
fn test_get_tokens() {
    let content_example = String::from("print(\"Hello, World\");");
    let mut lex = Lexer::new(content_example);
    let result: Vec<Token> = lex.get_tokens();

    let expected_result: Vec<Token> = vec![
        Token { token_value: String::from("\0"), token_type: TokenType::EOF},
        Token { token_value: String::from("print"), token_type: TokenType::IDENTIFIER },
        Token { token_value: String::from("("), token_type: TokenType::LPAREN },
        Token { token_value: String::from("\""), token_type: TokenType::DQUOTES },
        Token { token_value: String::from("Hello"), token_type: TokenType::IDENTIFIER },
        Token { token_value: String::from(","), token_type: TokenType::COMMA },
        Token { token_value: String::from("World"), token_type: TokenType::IDENTIFIER },
        Token { token_value: String::from("\""), token_type: TokenType::DQUOTES },
        Token { token_value: String::from(")"), token_type: TokenType::RPAREN },
        Token { token_value: String::from(";"), token_type: TokenType::SEMICOLON},
    ];

    assert_eq!(result, expected_result);
}
