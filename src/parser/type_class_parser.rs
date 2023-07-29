use chumsky::prelude::*;

use std::collections::HashMap;

use crate::parser::lexer::{Token, lexer};
use crate::parser::type_parser::{type_parser, type_statement_parser};
use crate::interpreter::Interpreter;
use crate::types::{Type, Value, TypeUtils};


pub struct TypeClass {
    pub name: Type,
    pub functions: Vec<Result<Type,(String,Value)>>,
}

pub fn function_prototype_parser() -> impl Parser<Token, Result<Type,(String,Value)>, Error = Simple<Token>> {
    let infix_function_without_effects = just(Token::Function)
        .then_ignore(just(Token::ParenLeft))
        .then(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        }))
        .then_ignore(just(Token::ParenRight))
        .then(type_parser()
              .separated_by(just(Token::Comma))
              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight)))
        .then_ignore(just(Token::FunctionReturn))
        .then(type_parser())
        .map(|(((_, name), args), return_type)| {
            Value::Function(false, args.iter().map(|x| (String::new(), Some(x.clone()))).collect(), Vec::new(), return_type, HashMap::new(), String::new())
        });


    let infix_function_with_effects = just(Token::Function)
        .then_ignore(just(Token::ParenLeft))
        .then(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        }))
        .then_ignore(just(Token::ParenRight))
        .then(type_parser()
              .separated_by(just(Token::Comma))
              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight)))
        .then(type_parser()
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::Identifier("<".to_string())), just(Token::Identifier(">".to_string()))))
        .then_ignore(just(Token::FunctionReturn))
        .then(type_parser())
        .map(|(((_, name), args), return_type)| {
            Value::Function(false, args.iter().map(|x| (String::new(), Some(x.clone()))).collect(), Vec::new(), return_type, HashMap::new(), String::new())
        });


    

    let infix_function = choice((
        infix_function_without_effects,
        infix_function_with_effects,
    ))
    .map(|x| Ok(x.get_type()));


    let normal_function_without_effects = just(Token::Function)
        .then(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        }))
        .then(type_parser()
              .separated_by(just(Token::Comma))
              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight)))
        .then_ignore(just(Token::FunctionReturn))
        .then(type_parser())
        .map(|(((_, name), args), return_type)| {
            Value::Function(false, args.iter().map(|x| (String::new(), Some(x.clone()))).collect(), Vec::new(), return_type, HashMap::new(), String::new())
        });


    let normal_function_with_effects = just(Token::Function)
        .then(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        }))
        .then(type_parser()
              .separated_by(just(Token::Comma))
              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight)))
        .then(type_parser()
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::Identifier("<".to_string())), just(Token::Identifier(">".to_string()))))
        .then_ignore(just(Token::FunctionReturn))
        .then(type_parser())
        .map(|(((_, name), args), return_type)| {
            Value::Function(false, args.iter().map(|x| (String::new(), Some(x.clone()))).collect(), Vec::new(), return_type, HashMap::new(), String::new())
        });

    let normal_function = choice((
        normal_function_without_effects,
        normal_function_with_effects,
    ))
    .map(|x| Ok(x.get_type()));
    

    choice((
        infix_function,
        normal_function,
    ))
}


pub fn type_class_definition_parser() -> impl Parser<Token, TypeClass, Error = Simple<Token>> {
    
    let type_class_parser = just(Token::Class)
        .ignore_then(type_parser())
        .then_ignore(just(Token::CurlyLeft))
        .then(function_prototype_parser().repeated())
        .then_ignore(just(Token::CurlyRight))
        .map(|(type_name, functions)| {
            TypeClass{ name:type_name, functions}
        });


    type_class_parser
}

