use chumsky::prelude::*;

use crate::parser::lexer::{Token, lexer};


use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    TypeList {
        name: Box<Type>,
        parameters: Vec<Type>,
    },
    Function {
        parameters: Vec<Type>,
        effects: Vec<Type>,
        return_type: Box<Type>,
    },
    Single(String),
    Tuple(Vec<Type>),
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Single(name) => write!(f, "{}", name),
            Type::Tuple(types) => {
                let output = String::new();
                let mut output = output + "(";
                for t in types {
                    output += &format!("{}, ", t);
                }
                output.pop();
                output.pop();
                output += ")";
                write!(f, "{}", output)
            },
            Type::Function{parameters, effects, return_type} => {
                let mut output = "fn(".to_string();

                for p in parameters {
                    output += &format!("{}, ", p);
                }
                output.pop();
                output.pop();
                output += ")";
                for e in effects {
                    output += &format!("{} ", e);
                }
                output += "-> ";
                output += &format!("{}", return_type);
                write!(f, "{}", output)
            },
            Type::TypeList{name, parameters} => {
                let mut output = format!("({}", name);
                for p in parameters {
                    output += &format!(" {}", p);
                }
                output += ")";
                write!(f, "{}", output)
            }
            
        }
    }

}


pub fn type_parser() -> impl Parser<Token, Type, Error = Simple<Token>> {

    let single_type = filter_map(|span, token| match token {
        Token::Identifier(value) => Ok(Type::Single(value)),
        _ => Err(Simple::custom(span, format!("Expected identifier, found {:?}", token))),
    });

    let tuple_or_single = recursive(|tuple| tuple
                          .separated_by(just(Token::Comma))
                          .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                        .map(|types| Type::Tuple(types))
                                    .or(single_type.clone())
    );
    let empty_tuple = just(Token::ParenLeft).then(just(Token::ParenRight)).map(|_| Type::Tuple(vec![]));
    
    let tuple = choice((empty_tuple, tuple_or_single));

    let type_list = tuple.clone()
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::Identifier("<".to_string())), just(Token::Identifier(">".to_string())));

    let type_group = filter_map(|span, token| match token {
        Token::Identifier(value) => Ok(Type::Single(value)),
        _ => Err(Simple::custom(span, format!("Expected identifier, found {:?}", token))),
    }).then(type_list.clone()).map(|(name, parameters)| Type::TypeList{name: Box::new(name), parameters});

    /*let type_group = recursive(|tg|
                               filter_map(|span, token| match token {
                                      Token::Identifier(value) => Ok(Type::Single(value)),
                                      _ => Err(Simple::custom(span, format!("Expected identifier, found {:?}", token))),
                               })
                               .then(tg
                                     .separated_by(just(Token::Comma))
                                     .delimited_by(just(Token::Identifier("<".to_string())), just(Token::Identifier(">".to_string())))
                                     .map(|vec| vec))
                            .map(|(name, parameters)| Type::TypeList{name: Box::new(name), parameters})
    );*/


    /*let type_group = recursive(|tg| tg
        .then(
            filter_map(|span, token| match token {
                    Token::Identifier(value) => Ok(Type::Single(value)),
                    _ => Err(Simple::custom(span, format!("Expected identifier, found {:?}", token))),
            })
        )
                               .then(type_list.clone())
                            .map(|((name, parameters), parameters2)| Type::TypeList{name: Box::new(name), parameters: parameters2}));*/
                               
                               
                               
    
    let basic_types = choice((type_group, tuple));
    
    let function_args = basic_types.clone()
        .separated_by(just(Token::Comma));

    let function_effects = type_list;

    let function = just(Token::Function)
        .then_ignore(just(Token::ParenLeft))
        .then(function_args)
        .then_ignore(just(Token::ParenRight))
        .then(function_effects)
        .then_ignore(just(Token::FunctionReturn))
        .then(basic_types.clone())
        .map(|(((_, parameters), effects), return_type)| Type::Function{parameters, effects, return_type: Box::new(return_type)});

    choice((function, basic_types))

}


#[cfg(test)]
mod type_parser_tests {
    use super::*;

    #[test]
    fn test_single_type() {
        let result = type_parser().parse(lexer("Int").unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse single type: Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Single("Int".to_string()), "Failed to parse single type: Int");
    }

    #[test]
    fn test_tuple() {
        let result = type_parser().parse(lexer("(Int, Int)").unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse tuple: (Int, Int)");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Tuple(vec![Type::Single("Int".to_string()), Type::Single("Int".to_string())]), "Failed to parse tuple: (Int, Int)");
    }

    #[test]
    fn test_nested_tuples() {
        let result = type_parser().parse(lexer("(Int, (Int, Int))").unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse nested tuples: (Int, (Int, Int))");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Tuple(vec![Type::Single("Int".to_string()), Type::Tuple(vec![Type::Single("Int".to_string()), Type::Single("Int".to_string())])]), "Failed to parse nested tuples: (Int, (Int, Int))");
    }

    #[test]
    fn test_empty_tuple() {
        let result = type_parser().parse(lexer("()").unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse empty tuple: ()");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Tuple(vec![]), "Failed to parse empty tuple: ()");
    }

    #[test]
    fn test_type_list() {
        let result = type_parser().parse(lexer("List<Int>").unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse type list: List<Int>");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::TypeList { name: Box::new(Type::Single("List".to_string())), parameters: vec![Type::Single("Int".to_string())] }, "Failed to parse type list: List<Int>");
    }

    #[test]
    fn test_nested_type_list() {
        let result = type_parser().parse(lexer("List<List<Int>>").unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse nested type list: List<List<Int>>");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::TypeList { name: Box::new(Type::Single("List".to_string())), parameters: vec![Type::TypeList { name: Box::new(Type::Single("List".to_string())), parameters: vec![Type::Single("Int".to_string())] }] }, "Failed to parse nested type list: List<List<Int>>");
    }
    

}
