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
    expected: Vec<TokenKind>,
}

impl Program {
    pub fn from_tokens(
        mut tokens: impl Iterator<Item = Token>,
        literal_data: &LiteralData,
    ) -> Result<Self, ParseError> {
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        let t = tokens.next();

        match t.as_ref().map(Token::kind) {
            Some(TokenKind::FunctionDefinition) => {
                functions.push(parse_function(&mut tokens, &literal_data)?)
            }
            Some(TokenKind::Struct) => structs.push(parse_struct(&mut tokens, &literal_data)?),
            None => (),
            _ => {
                return Err(ParseError {
                    expected: vec![TokenKind::FunctionDefinition, TokenKind::Struct],
                })
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
    let struct_name = if let Some(name) = tokens
        .next()
        .map(|t| literal_data.try_get_identifier(&t))
        .flatten()
    {
        name
    } else {
        return Err(ParseError {
            expected: vec![TokenKind::Identifier],
        });
    };

    Ok(Structure {
        name: struct_name.clone(),
        fields: Vec::new(),
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
