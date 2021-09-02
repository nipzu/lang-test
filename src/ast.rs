use crate::token::{Token, TokenKind};

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
    pub fn from_tokens(mut tokens: impl Iterator<Item = Token>) -> Result<Self, ParseError> {
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        let t = tokens.next();

        match t.as_ref().map(Token::kind) {
            Some(TokenKind::FunctionDefinition) => functions.push(parse_function(&mut tokens)?),
            Some(TokenKind::Struct) => structs.push(parse_struct(&mut tokens)?),
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

fn parse_function(tokens: &mut impl Iterator<Item = Token>) -> Result<Function, ParseError> {
    todo!()
}

fn parse_struct(tokens: &mut impl Iterator<Item = Token>) -> Result<Structure, ParseError> {
    let struct_name_token = tokens.next();

    let struct_name =
        if let Some(TokenKind::Identifier(ident)) = struct_name_token.as_ref().map(Token::kind) {
            ident
        } else {
            return Err(ParseError {
                expected: vec![TokenKind::Identifier(String::new())],
            });
        };

    todo!()
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
