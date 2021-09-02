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

pub fn tokenize_text(contents: &str) -> Result<Vec<Token>, TokenizingError> {
    let mut chars = CharLocationScanner::new(contents);
    let mut tokens = Vec::new();

    while let Some((c, location)) = chars.current_char_and_location() {
        tokens.push(Token::new(
            match c {
                _ if c.is_whitespace() => {
                    chars.advance();
                    continue;
                }
                'a'..='z' | 'A'..='Z' | '_' => tokenize_identifier_or_keyword(&mut chars),
                '0'..='9' => tokenize_integer(&mut chars)?,
                '"' => tokenize_string(&mut chars)?,
                _ => tokenize_other_token(&mut chars).ok_or(TokenizingError {
                    location,
                    kind: TokenizingErrorKind::UnknownToken,
                })?,
            },
            location,
        ));
    }

    Ok(tokens)
}

// first char should be an ascii letter or underscore
fn tokenize_identifier_or_keyword(chars: &mut CharLocationScanner) -> TokenKind {
    assert!(matches!(
        chars.current_char(),
        Some('A'..='Z' | 'a'..='z' | '_')
    ));

    let mut token_chars = Vec::new();
    while let Some(c @ ('A'..='Z' | 'a'..='z' | '0'..='9' | '_')) = chars.current_char() {
        token_chars.push(c as u8);
        chars.advance();
    }

    let s = String::from_utf8(token_chars).unwrap();
    let token_kind = match s.as_str() {
        "fn" => TokenKind::FunctionDefinition,
        "mut" => TokenKind::Mutable,
        "struct" => TokenKind::Struct,
        _ => TokenKind::Identifier(s),
    };
    token_kind
}

fn tokenize_integer(chars: &mut CharLocationScanner) -> Result<TokenKind, TokenizingError> {
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

    Ok(TokenKind::IntegerLiteral(digits))
}

fn tokenize_string(chars: &mut CharLocationScanner) -> Result<TokenKind, TokenizingError> {
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
    Ok(TokenKind::StringLiteral(string))
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
            TokenKind::Identifier("hello".into()),
            TokenKind::Comma,
            TokenKind::Identifier("world".into()),
            TokenKind::Not,
        ];

        assert!(tokenize_text(input)
            .unwrap()
            .iter()
            .map(Token::kind)
            .eq(output.iter()));
    }

    #[test]
    fn test2() {
        let input = "=:=";

        assert!(tokenize_text(input).is_err());
    }

    #[test]
    fn test_new_line_between() {
        let input1 = "=\n=";
        let expected_output1 = &[&TokenKind::Assign, &TokenKind::Assign];
        let output1 = tokenize_text(input1).unwrap();
        assert_eq!(
            output1.iter().map(Token::kind).collect::<Vec<_>>(),
            expected_output1
        );

        let input2 = "first\nsecond";
        let expected_output2 = &[
            &TokenKind::Identifier("first".into()),
            &TokenKind::Identifier("second".into()),
        ];
        let output2 = tokenize_text(input2).unwrap();
        assert_eq!(
            output2.iter().map(Token::kind).collect::<Vec<_>>(),
            expected_output2
        );
    }
}
