use std::iter::Peekable;
use std::vec::IntoIter;

use crate::token::{Token, TokenKind};
use crate::tokenizer::LiteralData;

#[derive(Debug)]
pub struct Program {
    functions: Vec<Function>,
    structs: Vec<Structure>,
}

pub struct AbstractSyntaxTree {
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct ParseError {
    token: Option<Token>,
    expected: Vec<TokenKind>,
}

type TokenIter = Peekable<IntoIter<Token>>;

impl Program {
    pub fn from_tokens(tokens: Vec<Token>, literal_data: LiteralData) -> Result<Self, ParseError> {
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        let mut tokens = tokens.into_iter().peekable();

        while let Some(token) = tokens.next() {
            match token.kind() {
                TokenKind::FunctionDefinition => {
                    functions.push(parse_function(&mut tokens, &literal_data)?);
                }
                TokenKind::Struct => structs.push(parse_struct(&mut tokens, &literal_data)?),
                _ => {
                    err_expected(
                        Some(token),
                        &[TokenKind::FunctionDefinition, TokenKind::Struct],
                    )?;
                }
            }
        }

        Ok(Self { functions, structs })
    }
}

fn parse_function(
    tokens: &mut TokenIter,
    literal_data: &LiteralData,
) -> Result<Function, ParseError> {
    let name = expect_identifier(tokens, literal_data)?.clone();

    expect_token(tokens, TokenKind::OpenParentheses)?;
    let arguments = parse_value_type_list(tokens, literal_data, TokenKind::CloseParentheses)?;

    let next_token = tokens.next();
    let return_type = match next_token.as_ref().map(Token::kind) {
        Some(TokenKind::OpenBraces) => None,
        Some(TokenKind::RightArrow) => {
            let type_name = expect_identifier(tokens, literal_data)?;
            expect_token(tokens, TokenKind::OpenBraces)?;
            Some(type_name.clone())
        }
        _ => err_expected(next_token, &[TokenKind::RightArrow, TokenKind::OpenBraces])?,
    };

    let body = parse_code_block(tokens, literal_data)?;

    Ok(Function {
        name,
        arguments,
        return_type,
        body,
    })
}

fn parse_struct(
    tokens: &mut TokenIter,
    literal_data: &LiteralData,
) -> Result<Structure, ParseError> {
    let struct_name = expect_identifier(tokens, literal_data)?;

    expect_token(tokens, TokenKind::OpenBraces)?;
    let fields = parse_value_type_list(tokens, literal_data, TokenKind::CloseBraces)?;

    Ok(Structure {
        name: struct_name.clone(),
        fields,
    })
}

fn parse_value_type_list(
    tokens: &mut TokenIter,
    literal_data: &LiteralData,
    end_token: TokenKind,
) -> Result<Vec<(String, String)>, ParseError> {
    let mut list = Vec::new();

    match tokens.peek().map(Token::kind) {
        Some(TokenKind::Identifier) => (),
        Some(kind) if kind == end_token => {
            tokens.next();
            return Ok(list);
        }
        _ => err_expected(tokens.next(), &[TokenKind::Identifier, end_token])?,
    }

    loop {
        let value_name = expect_identifier(tokens, literal_data)?;
        expect_token(tokens, TokenKind::FieldTypeSeparator)?;
        let value_type = expect_identifier(tokens, literal_data)?;
        list.push((value_name.clone(), value_type.clone()));

        let next_token = tokens.next();
        match next_token.as_ref().map(Token::kind) {
            Some(TokenKind::Comma) => continue,
            Some(kind) if kind == end_token => break,
            _ => err_expected(next_token, &[TokenKind::Comma, end_token])?,
        }
    }

    Ok(list)
}

fn parse_code_block(
    tokens: &mut TokenIter,
    literal_data: &LiteralData,
) -> Result<CodeBlock, ParseError> {
    todo!()
}

fn parse_statement(
    tokens: &mut TokenIter,
    literal_data: &LiteralData,
) -> Result<Statement, ParseError> {
    todo!()
}

fn parse_expression(
    tokens: &mut TokenIter,
    literal_data: &LiteralData,
) -> Result<Statement, ParseError> {
    todo!()

    
    // -, !, return, indentifier, literal, *, & 
}

#[derive(Debug)]
struct Structure {
    name: String,
    fields: Vec<(String, String)>,
}

#[derive(Debug)]
struct Function {
    name: String,
    arguments: Vec<(String, String)>,
    return_type: Option<String>,
    body: CodeBlock,
}

#[derive(Debug)]
struct Statement {}

#[derive(Debug)]
struct CodeBlock {
    statements: Vec<Statement>,
}

// TODO: this could probably be used with the `?` in the future
fn err_expected(token: Option<Token>, expected: &[TokenKind]) -> Result<!, ParseError> {
    Err(ParseError {
        token,
        expected: expected.to_vec(),
    })
}

fn expect_token(tokens: &mut TokenIter, kind: TokenKind) -> Result<Token, ParseError> {
    let next_token = tokens.next();
    match next_token {
        Some(token) if token.kind() == kind => Ok(token),
        _ => err_expected(next_token, &[kind])?,
    }
}

fn expect_identifier<'a>(
    tokens: &mut TokenIter,
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
