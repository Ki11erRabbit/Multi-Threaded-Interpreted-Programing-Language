use chumsky::prelude::*;

use crate::parser::lexer::{Token, lexer};

use std::ops::Range;
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
    Unit,
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
            },
            Type::Unit => write!(f, "()"),
            
        }
    }

}


pub fn type_parser() -> impl Parser<Token, Type, Error = Simple<Token>> {

    let single_type = filter_map(|span: Range<usize> , token| match token {
        Token::Identifier(value) => Ok(Type::Single(value)),
        _ => Err(Simple::custom(span, format!("Expected identifier, found {:?}", token))),
    })
        .labelled("Single Type Parser");

    let tuple_or_single = recursive(|tuple| tuple
                          .separated_by(just(Token::Comma))
                          .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                        .map(|types| Type::Tuple(types))
                                    .or(single_type.clone())
    ).labelled("Tuple or Single Type Parser");
    //let empty_tuple = just(Token::ParenLeft).then(just(Token::ParenRight)).map(|_| Type::Tuple(vec![]));
    let empty_tuple = just(Token::Unit).map(|_| Type::Unit);
    
    let tuple = choice((empty_tuple, tuple_or_single));
    //let tuple = tuple_or_single.clone();

    
    /*let type_list = recursive(|tg| tg
                              .separated_by(just(Token::WhiteSpace))
                              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                              .map(|types: Vec<Type>| Type::TypeList{name: Box::new(types[0].clone()), parameters: types[1..].to_owned()})
                            .or(tuple.clone())
    );*/

    let type_list = recursive(|tg| tg.repeated()
                              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                            .map(|types: Vec<Type>| Type::TypeList{name: Box::new(types[0].clone()), parameters: types[1..].to_owned()})
                            .or(tuple.clone()));
                               
    
    //let basic_types = choice((type_list.clone(), tuple));
    //basic_types
    //type_list
    let function_args = type_list.clone()
        .separated_by(just(Token::Comma));

    let function_effects = type_list.clone()
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::Identifier("<".to_string())), just(Token::Identifier(">".to_string()))
        );

    let function_with_effects = just(Token::Function)
        .then_ignore(just(Token::ParenLeft))
        .then(function_args.clone())
        .then_ignore(just(Token::ParenRight))
        .then(function_effects)
        .then_ignore(just(Token::FunctionReturn))
        .then(type_list.clone())
        .map(|(((_, parameters), effects), return_type)| Type::Function{parameters, effects, return_type: Box::new(return_type)})
        .labelled("Function with Effects Parser");

    let function_without_effects = just(Token::Function)
        .then_ignore(just(Token::ParenLeft))
        .then(function_args)
        .then_ignore(just(Token::ParenRight))
        .then_ignore(just(Token::FunctionReturn))
        .then(type_list.clone())
        .map(|((_, parameters), return_type)| Type::Function{parameters, effects: vec![], return_type: Box::new(return_type)})
        .labelled("Function without Effects Parser");

    let function = choice((function_with_effects, function_without_effects));

    let escaped_function = just(Token::ParenLeft)
        .ignore_then(function.clone())
        .then_ignore(just(Token::ParenRight))
        .labelled("Escaped Function Parser");

    let function_or_type_list = choice((function, escaped_function ,type_list, single_type));

    let tuple_or_function = recursive(|tuple| tuple
                          .separated_by(just(Token::Comma))
                          .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                        .map(|types| Type::Tuple(types))
                                      .or(function_or_type_list)
    ).labelled("Tuple or Function Type Parser");

    
    let type_list = recursive(|tg| tg.repeated()
                              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                            .map(|types: Vec<Type>| Type::TypeList{name: Box::new(types[0].clone()), parameters: types[1..].to_owned()})
                              .or(tuple_or_function));

    /*let recursive_type_parser = recursive(|tg| choice((
        type_list.clone(),
        tg
            ))
    )
        .labelled("Recursive Type Parser");

    recursive_type_parser*/
    type_list
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

        assert_eq!(result, Type::Unit, "Failed to parse empty tuple: ()");
    }

    #[test]
    fn test_type_list() {

        let lexer_result = lexer("(List Int)");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex type list: (List Int)");
        }
        
        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse type list: (List Int)");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::TypeList { name: Box::new(Type::Single("List".to_string())), parameters: vec![Type::Single("Int".to_string())] }, "Failed to parse type list: (List Int)");
    }

    #[test]
    fn test_nested_type_list() {
        let lexer_result = lexer("(List (List Int))");
        
        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex type list: (List (List Int))");
        }
        
        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse nested type list: List<List<Int>>");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::TypeList { name: Box::new(Type::Single("List".to_string())), parameters: vec![Type::TypeList { name: Box::new(Type::Single("List".to_string())), parameters: vec![Type::Single("Int".to_string())] }] }, "Failed to parse nested type list: List<List<Int>>");
    }

    #[test]
    fn test_function_type() {
        let lexer_result = lexer("fn(Int) -> Int");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex function type: fn(Int) -> Int");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse function type: fn(Int) -> Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function { parameters: vec![Type::Single("Int".to_string())], effects: vec![], return_type: Box::new(Type::Single("Int".to_string())) }, "Failed to parse function type: fn(Int) -> Int");
    }

    #[test]
    fn test_function_effect() {
        let lexer_result = lexer("fn(Int) <exn> -> Int");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex function type: fn(Int) <exn> -> Int");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse function type: fn(Int) <exn> -> Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function { parameters: vec![Type::Single("Int".to_string())], effects: vec![Type::Single("exn".to_string())], return_type: Box::new(Type::Single("Int".to_string())) }, "Failed to parse function type: fn(Int) <exn> -> Int");
    }

    #[test]
    fn test_function_many_effects() {
        let lexer_result = lexer("fn(Int) <exn, io> -> Int");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex function type: fn(Int) <exn, io> -> Int");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse function type: fn(Int) <exn, io> -> Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function { parameters: vec![Type::Single("Int".to_string())], effects: vec![Type::Single("exn".to_string()), Type::Single("io".to_string())], return_type: Box::new(Type::Single("Int".to_string())) }, "Failed to parse function type: fn(Int) <exn, io> -> Int");
    }

    #[test]
    fn test_complicated_tuple() {
        let lexer_result = lexer("(Int, (Int, Int), (List Int), fn(Int) -> Int)");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex complicated tuple: (Int, (Int, Int), (List Int), fn(Int) -> Int)");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse complicated tuple: (Int, (Int, Int), (List Int), fn(Int) -> Int)");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Tuple(vec![
            Type::Single("Int".to_string()),
            Type::Tuple(vec![
                Type::Single("Int".to_string()),
                Type::Single("Int".to_string())
            ]),
            Type::TypeList {
                name: Box::new(Type::Single("List".to_string())),
                parameters: vec![Type::Single("Int".to_string())]
            },
            Type::Function {
                parameters: vec![Type::Single("Int".to_string())],
                effects: vec![],
                return_type: Box::new(Type::Single("Int".to_string()))
            }
        ]), "Failed to parse complicated tuple: (Int, (Int, Int), (List Int), fn(Int) -> Int)");
    }


    #[test]
    fn test_complicated_function() {
        let lexer_result = lexer("fn(Int, (Int, Int), (List Int), fn(Int) -> Int) -> Int");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex complicated function: fn(Int, (Int, Int), (List Int), fn(Int) -> Int) -> Int");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse complicated function: fn(Int, (Int, Int), (List Int), fn(Int) -> Int) -> Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function {
            parameters: vec![
                Type::Single("Int".to_string()),
                Type::Tuple(vec![
                    Type::Single("Int".to_string()),
                    Type::Single("Int".to_string())
                ]),
                Type::TypeList {
                    name: Box::new(Type::Single("List".to_string())),
                    parameters: vec![Type::Single("Int".to_string())]
                },
                Type::Function {
                    parameters: vec![Type::Single("Int".to_string())],
                    effects: vec![],
                    return_type: Box::new(Type::Single("Int".to_string()))
                }
            ],
            effects: vec![],
            return_type: Box::new(Type::Single("Int".to_string()))
        }, "Failed to parse complicated function: fn(Int, (Int, Int), (List Int), fn(Int) -> Int) -> Int");
    }

    #[test]
    fn test_complicated_type_list() {
        let lexer_result = lexer("(List fn((Int, Int)) -> Int fn(Int) -> Int)");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex complicated type list: List fn((Int, Int)) -> Int fn(Int) -> Int");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse complicated type list: List fn((Int, Int)) -> Int fn(Int) -> Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::TypeList {
            name: Box::new(Type::Single("List".to_string())),
            parameters: vec![
                Type::Function {
                    parameters: vec![Type::Tuple(vec![
                        Type::Single("Int".to_string()),
                        Type::Single("Int".to_string())
                    ])],
                    effects: vec![],
                    return_type: Box::new(Type::Single("Int".to_string()))
                },
                Type::Function {
                    parameters: vec![Type::Single("Int".to_string())],
                    effects: vec![],
                    return_type: Box::new(Type::Single("Int".to_string()))
                }
            ]
        }, "Failed to parse complicated type list: List fn((Int, Int)) -> Int fn(Int) -> Int");
    }

}
