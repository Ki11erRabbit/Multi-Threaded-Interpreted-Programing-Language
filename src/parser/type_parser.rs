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

    let single_or_unit = filter_map(|span: Range<usize> , token| match token {
        Token::Identifier(value) => Ok(Type::Single(value)),
        Token::Unit => Ok(Type::Unit),
        _ => Err(Simple::custom(span, format!("Expected identifier, found {:?}", token))),
    })
        .labelled("Single or Unit Type Parser");


    let tuple = recursive(|tuple| tuple
                          .separated_by(just(Token::Comma))
                          .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                        .map(|types| Type::Tuple(types))
                                    .or(type_parser())
    ).labelled("Tuple or Single Type Parser");
    //let empty_tuple = just(Token::Unit).map(|_| Type::Unit);
    
    //let tuple = choice((empty_tuple, tuple_or_single));

    let type_list = recursive(|tg| tg.repeated()
                              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                            .map(|types: Vec<Type>| Type::TypeList{name: Box::new(types[0].clone()), parameters: types[1..].to_owned()})
                            .or(type_parser()));
                               
    
    
    
    /*let function_args = type_list.clone()
        .separated_by(just(Token::Comma));*/

    let function_effects = type_list.clone()
        .separated_by(just(Token::Comma))
        .delimited_by(just(Token::Identifier("<".to_string())), just(Token::Identifier(">".to_string()))
        );


    let function_with_effects = recursive(|fun|
                                          just(Token::Function)
                                          .then_ignore(just(Token::ParenLeft))
                                          .then(fun.clone().separated_by(just(Token::Comma)))
                                          .then_ignore(just(Token::ParenRight))
                                          .then(function_effects.clone())
                                          .then_ignore(just(Token::FunctionReturn))
                                          .then(type_list.clone().or(fun))
                                          .map(|(((_, parameters), effects), return_type)| Type::Function{parameters, effects, return_type: Box::new(return_type)})
                                        .or(type_parser())
    );

    let function_without_effects = recursive(|fun| 
                                          just(Token::Function)
                                          .then_ignore(just(Token::ParenLeft))
                                          .then(fun.clone().separated_by(just(Token::Comma)))
                                          .then_ignore(just(Token::ParenRight))
                                          .then_ignore(just(Token::FunctionReturn))
                                             .then(type_list.clone().or(fun))
                                             .map(|((_, parameters), return_type)| Type::Function{parameters, effects: vec![], return_type: Box::new(return_type)})
                                        .or(type_parser())
    );


    let function = choice((function_with_effects, function_without_effects));

    choice((tuple,single_or_unit, function, type_list))

    /*let escaped_function = just(Token::ParenLeft)
        .ignore_then(function.clone())
        .then_ignore(just(Token::ParenRight))
        .labelled("Escaped Function Parser");*/

    //let function_or_type_list = choice((function,escaped_function,type_list, single_type));

    /*let tuple_or_function = recursive(|tuple| tuple
                          .separated_by(just(Token::Comma))
                          .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                        .map(|types| Type::Tuple(types))
                                      .or(function_or_type_list)
    ).labelled("Tuple or Function Type Parser");

    
    let type_list = recursive(|tg| tg.repeated()
                              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
                            .map(|types: Vec<Type>| Type::TypeList{name: Box::new(types[0].clone()), parameters: types[1..].to_owned()})
                              .or(tuple_or_function));

    type_list*/

    //function_or_type_list
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

    #[test]
    fn test_function_returns_tuple() {
        let lexer_result = lexer("fn(String) -> (Char, Float)");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex function returns tuple: fn(String) -> (Char, Float)");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse function returns tuple: fn(Int) -> (Int, Int)");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function {
            parameters: vec![Type::Single("String".to_string())],
            effects: vec![],
            return_type: Box::new(Type::Tuple(vec![
                Type::Single("Char".to_string()),
                Type::Single("Float".to_string())
            ]))
        }, "Failed to parse function returns tuple: fn(Int) -> (Int, Int)");
    }

    #[test]
    fn test_function_returns_function() {
        let lexer_result = lexer("fn(String) -> fn(Int) -> Int");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex function returns function: fn(Int) -> fn(Int) -> Int");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse function returns function: fn(Int) -> fn(Int) -> Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function {
            parameters: vec![Type::Single("String".to_string())],
            effects: vec![],
            return_type: Box::new(Type::Function {
                parameters: vec![Type::Single("Int".to_string())],
                effects: vec![],
                return_type: Box::new(Type::Single("Int".to_string()))
            })
        }, "Failed to parse function returns function: fn(Int) -> fn(Int) -> Int");
    }

    #[test]
    fn test_mixed_type() {
        let lexer_result = lexer("fn((Int,(List Char))) -> (Float, Char)");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex mixed type statement: fn((Int,(List Int))) -> (Int, Int)");
        }

        let result = type_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse mixed type statement: fn((Int,(List Int))) -> (Int, Int)");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function {
            parameters: vec![Type::Tuple(vec![
                Type::Single("Int".to_string()),
                Type::Tuple(vec![
                    Type::Single("List".to_string()),
                    Type::Single("Int".to_string())
                ])
            ])],
            effects: vec![],
            return_type: Box::new(Type::Tuple(vec![
                Type::Single("Int".to_string()),
                Type::Single("Int".to_string())
            ]))
        }, "Failed to parse mixed type statement: x: fn((Int,(List Int))) -> (Int, Int)");
    }

}


pub fn type_statement_parser() -> impl Parser<Token, Type, Error = Simple<Token>> {

    just(Token::Colon).ignore_then(type_parser())
}


#[cfg(test)]
mod type_statement_parser {
    use super::*;

    #[test]
    fn test_type_statement() {
        let lexer_result = lexer(": Int");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex type statement: x: Int");
        }

        let result = type_statement_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse type statement: x: Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Single("Int".to_string()), "Failed to parse type statement: x: Int");
    }

    #[test]
    fn test_function_type_statement() {
        let lexer_result = lexer(": fn(Int) -> Int");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex function type statement: x: fn(Int) -> Int");
        }

        let result = type_statement_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse function type statement: x: fn(Int) -> Int");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function {
            parameters: vec![Type::Single("Int".to_string())],
            effects: vec![],
            return_type: Box::new(Type::Single("Int".to_string()))
        }, "Failed to parse function type statement: x: fn(Int) -> Int");
    }

    #[test]
    fn test_tuple_type_statement() {
        let lexer_result = lexer(": (Int, Int)");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex tuple type statement: x: (Int, Int)");
        }

        let result = type_statement_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse tuple type statement: x: (Int, Int)");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Tuple(vec![
            Type::Single("Int".to_string()),
            Type::Single("Int".to_string())
        ]), "Failed to parse tuple type statement: x: (Int, Int)");
    }

    #[test]
    fn test_type_list_type_statement() {
        let lexer_result = lexer(": (List Int)");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex type list type statement: x: (List Int)");
        }

        let result = type_statement_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse type list type statement: x: (List Int)");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::TypeList {
            name: Box::new(Type::Single("List".to_string())),
            parameters: vec![Type::Single("Int".to_string())]
        }, "Failed to parse type list type statement: x: (List Int)");
    }

    #[test]
    fn test_mixed_type_statement() {
        let lexer_result = lexer(": fn((String,(List Int))) -> (Char, Float)");

        if lexer_result.is_err() {
            eprintln!("{:?}", lexer_result);
            assert!(false, "Failed to lex mixed type statement: x: fn((String,(List Int))) -> (Char, Float)");
        }

        let result = type_statement_parser().parse(lexer_result.unwrap());

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failed to parse mixed type statement: x: fn((String,(List Int))) -> (Char, Float)");
        }

        let result = result.unwrap();

        assert_eq!(result, Type::Function {
            parameters: vec![Type::Tuple(vec![
                Type::Single("String".to_string()),
                Type::Tuple(vec![
                    Type::Single("List".to_string()),
                    Type::Single("Int".to_string())
                ])
            ])],
            effects: vec![],
            return_type: Box::new(Type::Tuple(vec![
                Type::Single("Char".to_string()),
                Type::Single("Float".to_string())
            ]))
        }, "Failed to parse mixed type statement: x: fn((String,(List Int))) -> (Char, Float)");
    }

}
        
        
    
    
