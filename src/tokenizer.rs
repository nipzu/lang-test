use crate::token::{Location, Token, TokenKind};
use std::collections::HashMap;
use std::lazy::SyncLazy;
use std::str::Chars;

const OTHER_TOKENS: [(&[char], TokenKind); 23] = [
    (&['{'], TokenKind::OpenBraces),
    (&['}'], TokenKind::CloseBraces),
    (&['('], TokenKind::OpenParentheses),
    (&[')'], TokenKind::CloseParentheses),
    (&['+'], TokenKind::Plus),
    (&['-'], TokenKind::Minus),
    (&['*'], TokenKind::Mul),
    (&['/'], TokenKind::Div),
    (&['%'], TokenKind::Rem),
    (&['='], TokenKind::Assign),
    (&[':', '='], TokenKind::DefineVar),
    (&[':'], TokenKind::FieldTypeSeparator),
    (&['=', '='], TokenKind::Equal),
    (&['>'], TokenKind::Greater),
    (&['>', '='], TokenKind::GreaterOrEqual),
    (&['<'], TokenKind::Less),
    (&['<', '='], TokenKind::LessOrEqual),
    (&['!'], TokenKind::Not),
    (&['|'], TokenKind::Or),
    (&['&'], TokenKind::And),
    (&['^'], TokenKind::Xor),
    (&[','], TokenKind::Comma),
    (&[';'], TokenKind::EndOfStatement),
];

static TOKEN_MAP: SyncLazy<HashMap<&[char], Option<TokenKind>>> = SyncLazy::new(|| {
    let mut token_map = HashMap::new();
    for (token_chars, token) in OTHER_TOKENS {
        for l in 0..token_chars.len() - 1 {
            token_map.entry(&token_chars[..l]).or_insert(None);
        }
        token_map.insert(token_chars, Some(token));
    }
    token_map
});

pub struct LiteralData {
    identifiers: HashMap<Location, String>,
    integer_literals: HashMap<Location, String>,
    string_literals: HashMap<Location, String>,
}

impl LiteralData {
    pub fn try_get_identifier(&self, token: &Token) -> Option<&String> {
        (*token.kind() == TokenKind::Identifier)
            .then(|| self.identifiers.get(token.location()))
            .flatten()
    }

    pub fn try_get_string_literal(&self, token: &Token) -> Option<&String> {
        (*token.kind() == TokenKind::StringLiteral)
            .then(|| self.string_literals.get(token.location()))
            .flatten()
    }

    pub fn try_get_integer_literal(&self, token: &Token) -> Option<&String> {
        (*token.kind() == TokenKind::IntegerLiteral)
            .then(|| self.integer_literals.get(token.location()))
            .flatten()
    }
}

#[derive(Clone)]
struct CharLocationScanner<'a> {
    cur_location: Location,
    cur_char: Option<char>,
    chars: Chars<'a>,
}

impl<'a> CharLocationScanner<'a> {
    fn new(contents: &'a str) -> CharLocationScanner<'a> {
        let mut chars = contents.chars();
        CharLocationScanner {
            cur_location: Location { line: 1, column: 1 },
            cur_char: chars.next(),
            chars,
        }
    }

    const fn current_char(&self) -> Option<char> {
        self.cur_char
    }

    const fn current_location(&self) -> Location {
        self.cur_location
    }

    fn current_char_and_location(&self) -> Option<(char, Location)> {
        self.cur_char.map(|c| (c, self.cur_location))
    }

    fn advance(&mut self) {
        if self.cur_char == Some('\n') {
            self.cur_location.line += 1;
            self.cur_location.column = 0;
        }
        self.cur_location.column += 1;
        self.cur_char = self.chars.next();
    }
}

pub fn tokenize_text(contents: &str) -> Result<(Vec<Token>, LiteralData), TokenizingError> {
    let mut chars = CharLocationScanner::new(contents);
    let mut tokens = Vec::new();
    let mut identifiers = HashMap::new();
    let mut string_literals = HashMap::new();
    let mut integer_literals = HashMap::new();

    while let Some((c, location)) = chars.current_char_and_location() {
        tokens.push(Token::new(
            match c {
                _ if c.is_whitespace() => {
                    chars.advance();
                    continue;
                }
                '#' => {
                    while let Some(c) = chars.current_char() {
                        chars.advance();
                        if c == '\n' {
                            break;
                        }
                    }
                    continue;
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let s = tokenize_identifier_or_keyword(&mut chars);
                    match s.as_str() {
                        "fn" => TokenKind::FunctionDefinition,
                        "mut" => TokenKind::Mutable,
                        "struct" => TokenKind::Struct,
                        _ => {
                            identifiers.insert(location, s);
                            TokenKind::Identifier
                        }
                    }
                }
                '0'..='9' => {
                    integer_literals.insert(location, tokenize_integer(&mut chars)?);
                    TokenKind::IntegerLiteral
                }
                '"' => {
                    string_literals.insert(location, tokenize_string(&mut chars)?);
                    TokenKind::StringLiteral
                }
                _ => tokenize_other_token(&mut chars).ok_or(TokenizingError {
                    location,
                    kind: TokenizingErrorKind::UnknownToken,
                })?,
            },
            location,
        ));
    }

    let literal_data = LiteralData {
        identifiers,
        string_literals,
        integer_literals,
    };

    Ok((tokens, literal_data))
}

// first char should be an ascii letter or underscore
fn tokenize_identifier_or_keyword(chars: &mut CharLocationScanner) -> String {
    assert!(matches!(
        chars.current_char(),
        Some('A'..='Z' | 'a'..='z' | '_')
    ));

    let mut token_chars = String::new();
    while let Some(c @ ('A'..='Z' | 'a'..='z' | '0'..='9' | '_')) = chars.current_char() {
        token_chars.push(c);
        chars.advance();
    }

    token_chars
}

fn tokenize_integer(chars: &mut CharLocationScanner) -> Result<String, TokenizingError> {
    assert!(matches!(chars.current_char(), Some('0'..='9')));

    let mut digits = String::new();

    // TODO: suffixes
    while let Some(c) = chars.current_char() {
        match c {
            '0'..='9' => digits.push(c),
            '_' => (),
            'A'..='Z' | 'a'..='z' => {
                return Err(TokenizingError {
                    location: chars.current_location(),
                    kind: TokenizingErrorKind::InvalidSuffix,
                })
            }
            _ => break,
        }

        chars.advance();
    }

    Ok(digits)
}

fn tokenize_string(chars: &mut CharLocationScanner) -> Result<String, TokenizingError> {
    assert_eq!(chars.current_char(), Some('"'));
    chars.advance();

    let mut string = String::new();
    while let Some(c) = chars.current_char() {
        string.push(match c {
            '\\' => {
                chars.advance();
                match chars.current_char() {
                    Some('"') => '"',
                    Some('\\') => '\\',
                    Some('n') => '\n',
                    Some('r') => '\r',
                    Some('t') => '\t',
                    _ => {
                        return Err(TokenizingError {
                            location: chars.current_location(),
                            kind: TokenizingErrorKind::InvalidEscape,
                        })
                    }
                }
            }
            '"' => {
                chars.advance();
                break;
            }
            _ => c,
        });
        chars.advance();
    }
    Ok(string)
}

fn tokenize_other_token(chars: &mut CharLocationScanner) -> Option<TokenKind> {
    // TODO: use stackvec or something
    let mut cur_chars = Vec::new();
    let mut cur_candidate = None;
    let mut continue_chars = chars.clone();
    while let Some(entry) = TOKEN_MAP.get(&cur_chars[..]) {
        if let Some(new_candidate) = entry {
            cur_candidate = Some(new_candidate.clone());
            continue_chars = chars.clone();
        }

        match chars.current_char() {
            Some(c) => cur_chars.push(c),
            None => break,
        }

        chars.advance();
    }

    *chars = continue_chars;

    cur_candidate
}

// TODO
#[derive(Debug)]
pub struct TokenizingError {
    pub location: Location,
    pub kind: TokenizingErrorKind,
}

#[derive(Debug)]
pub enum TokenizingErrorKind {
    InvalidSuffix,
    InvalidEscape,
    UnknownToken,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "hello, world!";
        let output = vec![
            TokenKind::Identifier,
            TokenKind::Comma,
            TokenKind::Identifier,
            TokenKind::Not,
        ];

        assert!(tokenize_text(input)
            .unwrap()
            .0
            .iter()
            .map(Token::kind)
            .eq(output.iter()));
    }

    #[test]
    fn test2() {
        let input = "=:=";

        assert!(tokenize_text(input).is_ok());
    }

    #[test]
    fn test_new_line_between() {
        let input1 = "=\n=";
        let expected_output1 = &[&TokenKind::Assign, &TokenKind::Assign];
        let output1 = tokenize_text(input1).unwrap();
        assert_eq!(
            output1.0.iter().map(Token::kind).collect::<Vec<_>>(),
            expected_output1
        );

        let input2 = "first\nsecond";
        let expected_output2 = &[&TokenKind::Identifier, &TokenKind::Identifier];
        let output2 = tokenize_text(input2).unwrap();
        assert_eq!(
            output2.0.iter().map(Token::kind).collect::<Vec<_>>(),
            expected_output2
        );
    }
}
