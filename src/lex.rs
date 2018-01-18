use token::{Token, TokenValue, Metadata};
use errors::*;

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
                .or(self.read_close_paren());
        }
        Ok(())
    }

    fn read_open_paren(&mut self) -> Result<()> {
        let mut char_iter = self.remaining_input.chars();
        if let Some(c) = char_iter.clone().peekable().peek() {
            if *c == '(' {
                char_iter.next();
                self.remaining_input = char_iter.as_str();
                let token = self.make_token(TokenValue::OpenParen);
                self.processed.push(token);
                self.column_number += 1;
            } else {
                self.remaining_input = char_iter.as_str();
                // err different token
            }
        } else {
            // err end of input
        }
        Ok(())
    }

    fn read_close_paren(&mut self) -> Result<()> {
        let mut char_iter = self.remaining_input.chars();
        if let Some(c) = char_iter.clone().peekable().peek() {
            if *c == ')' {
                char_iter.next();
                self.remaining_input = char_iter.as_str();
                let token = self.make_token(TokenValue::CloseParen);
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
