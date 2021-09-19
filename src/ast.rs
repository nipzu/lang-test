use crate::token::{Token, TokenKind};
use crate::tokenizer::LiteralData;

pub struct Program {
    functions: Vec<Function>,
    structs: Vec<Structure>,
}

pub struct AbstractSyntaxTree {
    statements: Vec<Statement>,
}

pub struct ParseError {
    token: Option<Token>,
    expected: Vec<TokenKind>,
}

impl Program {
    pub fn from_tokens(tokens: Vec<Token>, literal_data: LiteralData) -> Result<Self, ParseError> {
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        let mut tokens = tokens.into_iter().peekable();

        let t = tokens.next();

        match t.as_ref().map(Token::kind) {
            Some(TokenKind::FunctionDefinition) => {
                functions.push(parse_function(&mut tokens, &literal_data)?);
            }
            Some(TokenKind::Struct) => structs.push(parse_struct(&mut tokens, &literal_data)?),
            None => (),
            _ => {
                err_expected(t, &[TokenKind::FunctionDefinition, TokenKind::Struct])?;
            }
        }

        Ok(Self { functions, structs })
    }
}

fn parse_function(
    tokens: &mut impl Iterator<Item = Token>,
    literal_data: &LiteralData,
) -> Result<Function, ParseError> {
    todo!()
}

fn parse_struct(
    tokens: &mut impl Iterator<Item = Token>,
    literal_data: &LiteralData,
) -> Result<Structure, ParseError> {
    let struct_name = expect_identifier(tokens, literal_data)?;

    let mut fields = Vec::new();

    expect_token(tokens, TokenKind::OpenBraces)?;
    loop {
        let field_name = expect_identifier(tokens, literal_data)?;
        expect_token(tokens, TokenKind::FieldTypeSeparator)?;
        let field_type = expect_identifier(tokens, literal_data)?;
        fields.push((field_name.clone(), field_type.clone()));

        let next_token = tokens.next();
        match next_token.as_ref().map(Token::kind) {
            Some(TokenKind::Comma) => continue,
            Some(TokenKind::CloseBraces) => break,
            _ => err_expected(next_token, &[TokenKind::Comma, TokenKind::CloseBraces])?,
        }
    }

    Ok(Structure {
        name: struct_name.clone(),
        fields,
    })
}

struct Structure {
    name: String,
    fields: Vec<(String, String)>,
}

struct Function {
    return_type: String,
    arguments: Vec<(String, String)>,
}

struct Statement {}

// TODO: this could probably be used with the `?` in the future
fn err_expected(token: Option<Token>, expected: &[TokenKind]) -> Result<!, ParseError> {
    Err(ParseError {
        token,
        expected: expected.to_vec(),
    })
}

fn expect_token(
    tokens: &mut impl Iterator<Item = Token>,
    kind: TokenKind,
) -> Result<Token, ParseError> {
    let next_token = tokens.next();
    match next_token {
        Some(token) if token.kind() == kind => Ok(token),
        _ => err_expected(next_token, &[kind])?,
    }
}

fn expect_identifier<'a>(
    tokens: &mut impl Iterator<Item = Token>,
    literal_data: &'a LiteralData,
) -> Result<&'a String, ParseError> {
    let token = tokens.next();

    match token
        .as_ref()
        .and_then(|t| literal_data.try_get_identifier(t))
    {
        Some(name) => Ok(name),
        None => err_expected(token, &[TokenKind::Identifier])?,
    }
}
