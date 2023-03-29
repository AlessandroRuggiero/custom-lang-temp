pub mod token;

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '/'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0'
        }
    }

    pub fn peek_char (&self) -> char{
        if self.read_position >= self.input.len() {
            '0'
        } else {
            self.input[self.read_position]
        }
    }
    pub fn skip_char (& mut self) {
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }
    
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn skip_whitespace(&mut self) -> bool {
        let ch = self.ch;
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
            return true;
        }
        return false;
    }

    pub fn next_token(&mut self) -> token::Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_digit(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let tok: token::Token;
        while self.skip_whitespace() {}
        match self.ch {
            '=' => {
                tok = token::Token::ASSIGN;
            },
            '+' => {
                tok = token::Token::PLUS;
            },
            '-' => {
                if self.peek_char() == '>' { // we have a <-
                    tok = token::Token::GET;
                    self.skip_char();
                }else {
                    tok = token::Token::MINUS;
                }
            },
            '!' => {
                tok = token::Token::BANG;
            },
            '/' => {
                tok = token::Token::SLASH;
            },
            '*' => {
                tok = token::Token::ASTERISK;
            },
            '<' => {
                if self.peek_char() == '-' { // we have a <-
                    tok = token::Token::PUT;
                    self.skip_char();
                }else {
                    tok = token::Token::LT;
                }
            },
            '>' => {
                tok = token::Token::GT;
            },
            ';' => {
                tok = token::Token::SEMICOLON;
            },
            '(' => {
                tok = token::Token::LPAREN;
            },
            ')' => {
                tok = token::Token::RPAREN;
            },
            ',' => {
                tok = token::Token::COMMA;
            },
            '{' => {
                tok = token::Token::LBRACE;
            },
            '}' => {
                tok = token::Token::RBRACE;
            },
            '[' => {
                tok = token::Token::LBRAKET;
            },
            ']' => {
                tok = token::Token::RBRAKET;
            },
            '0' => {
                tok = token::Token::EOF;
            },
            '\"' => {
                tok = token::Token::DOUBLEQUOTES;
            }
            '.' => {
                tok = token::Token::DOT
            },
            _ => {
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        },
                        Err(_err) => {
                            return token::Token::IDENT(ident);
                        }
                    }
                } else if is_digit(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return token::Token::INT(ident);
                } 
                else {
                    println!("{}",self.ch == ' ');
                    return token::Token::ILLEGAL
                }
            }
        }
        self.read_char();
        tok
    }
}

impl Iterator for Lexer {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == token::Token::EOF {
            return None;
        } 
        return Some(token);
    }
}