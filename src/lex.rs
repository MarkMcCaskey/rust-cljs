use token::{Token, TokenValue, Metadata};
use errors::*;

macro_rules! read_token {
    ($fn_name:ident, $tok_char:expr, $tok_val:expr) => {
        fn $fn_name(&mut self) -> Result<()> {
            let mut char_iter = self.remaining_input.chars();
            if let Some(c) = char_iter.clone().peekable().peek() {
                if *c == $tok_char {
                    char_iter.next();
                    self.remaining_input = char_iter.as_str();
                    let token = self.make_token($tok_val);
                    self.processed.push(token);
                    self.column_number += 1;
                } else {
                    self.remaining_input = char_iter.as_str();
                    return Err(ErrorKind::UnrecognizedToken(self.line_number, self.column_number).into());
                }
            } else {
                return Err(ErrorKind::EndOfInput.into());
            }
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct LexContext<'a> {
    remaining_input: &'a str,
    processed: Vec<Token<'a>>,
    line_number: usize,
    column_number: usize,
}

// TODO: make a macro that takes a character, TokenValue, and name and creates a function
impl<'a> LexContext<'a> {
    pub fn create(input: &'a str) -> Self {
        LexContext {
            remaining_input: input,
            processed: Vec::new(),
            line_number: 1,
            column_number: 1,
        }
    }

    fn make_token(&self, token_value: TokenValue<'a>) -> Token<'a> {
        Token {
            value: token_value,
            metadata: Metadata {
                line: self.line_number,
                column: self.column_number,
            }
        }
    }

    pub fn lex(&mut self) -> Result<()> {
        while !self.remaining_input.is_empty() {
            self.consume_whitespace()
                .or(self.read_open_paren())
                .or(self.read_close_paren())
                .or(self.read_open_square_bracket())
                .or(self.read_close_square_bracket())
                .or(self.read_open_curly_brace())
                .or(self.read_close_curly_brace())
                .or(self.read_ampersand())
                .or(self.read_hash())
                .or(self.read_quote());
        }
        Ok(())
    }

    read_token!(read_open_paren,           '(',  TokenValue::OpenParen);
    read_token!(read_close_paren,          ')',  TokenValue::CloseParen);
    read_token!(read_open_square_bracket,  '[',  TokenValue::OpenSquareBracket);
    read_token!(read_close_square_bracket, ']',  TokenValue::CloseSquareBracket);
    read_token!(read_open_curly_brace,     '{',  TokenValue::OpenCurlyBrace);
    read_token!(read_close_curly_brace,    '}',  TokenValue::CloseCurlyBrace);
    read_token!(read_ampersand,            '&',  TokenValue::Amersand);
    read_token!(read_hash,                 '#',  TokenValue::Hash);
    read_token!(read_quote,                '\'', TokenValue::Quote);

    fn consume_whitespace(&mut self) -> Result<()> {
        let mut char_iter = self.remaining_input.chars();
        if let Some(c) = char_iter.clone().peekable().peek() {
            match *c {
                ' ' | '\r' => {
                    self.column_number += 1;
                },
                '\t' => {
                    // TODO: verify this makes sense (it probably doesn't)
                    self.column_number += 8;
                },
                '\n' => {
                    self.column_number = 0;
                    self.line_number += 1;
                }
                _ => return Err(ErrorKind::UnrecognizedToken(self.line_number, self.column_number).into()),
                //TODO: unicode whitespace, etc
            }
            char_iter.next();
            self.remaining_input = char_iter.as_str();
        } else {
            return Err(ErrorKind::EndOfInput.into());
        }
        Ok(())
    }
    
    
}
