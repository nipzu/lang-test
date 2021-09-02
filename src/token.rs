use std::fmt;

#[derive(Debug)]
pub struct Token {
    token_kind: TokenKind,
    location: Location,
}

impl Token {
    pub const fn new(token_kind: TokenKind, location: Location) -> Self {
        Self {
            token_kind,
            location,
        }
    }

    pub const fn kind(&self) -> &TokenKind {
        &self.token_kind
    }

    pub const fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    StringLiteral(String),
    IntegerLiteral(String),
    Identifier(String),
    OpenBraces,
    CloseBraces,
    OpenParentheses,
    CloseParentheses,
    Plus,
    Minus,
    Mul,
    Div,
    Rem,
    Assign,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    FunctionDefinition,
    Mutable,
    Struct,
    Not,
    Or,
    And,
    Xor,
    Comma,
    EndOfStatement,
}

// TODO: is this even ever used?
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::OpenBraces => "{",
            TokenKind::CloseBraces => "}",
            TokenKind::OpenParentheses => "(",
            TokenKind::CloseParentheses => ")",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Mul => "*",
            TokenKind::Div => "/",
            TokenKind::Rem => "%",
            TokenKind::Assign => "=",
            TokenKind::Equal => "==",
            TokenKind::Greater => ">",
            TokenKind::GreaterOrEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessOrEqual => "<=",
            TokenKind::FunctionDefinition => "fn",
            TokenKind::Mutable => "mut",
            TokenKind::Struct => "struct",
            TokenKind::Not => "!",
            TokenKind::Or => "|",
            TokenKind::And => "&",
            TokenKind::Xor => "^",
            TokenKind::Comma => ",",
            TokenKind::EndOfStatement => ";",
            TokenKind::StringLiteral(_) => unimplemented!(),
            TokenKind::Identifier(id_string) => id_string,
            TokenKind::IntegerLiteral(literal) => literal,
        };
        write!(f, "{}", s)
    }
}
