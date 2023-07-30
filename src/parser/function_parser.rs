use chumsky::prelude::*;
use lazy_static::lazy_static;

use crate::parser::lexer::Token;
use crate::parser::type_parser::{type_parser, type_statement_parser};
use crate::types::{Type, Value, TypeUtils};

use std::collections::HashMap;
use std::sync::RwLock;



#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    OperatorOrder(usize),// for setting infix function precedence
    RightAssociative,    // for setting infix function associativity
    LeftAssociative,     // for setting infix function associativity
    Atomic,              // for setting atomicity of variables
    ThreadLocal,         // for setting thread local variables. Not shared(Default)
    ThreadShared,        // for setting thread shared variables. Shared
    ThreadMutable,       // for setting thread mutable variables. Protected with Mutex
    ThreadSpawn,         // for setting if a function will be spawned in a new thread
    Minimal,             // for marking a type class function to be the only necessary function
    Default,             // for marking a type class function to use the default implementation
    Control,             // For defining an effect function that can alter control flow (i.e. exceptions)
    Final,               // For defining what function to call when an effect escapes into main
}


pub fn attribute_parser() -> impl Parser<Token, Vec<Attribute>, Error = Simple<Token>> {

    let basic_attribute = filter_map(|span, token| match token {
        Token::Identifier(name) => Ok(name),
        _ => Err(Simple::custom(span, "Expected identifier")),
    }
    )
        .then(filter_map(|span, token| match token {
            Token::Number(num) => Ok(num),
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier or number")),
        }).repeated())
        .map(|(name, values)| match name.as_str() {
            "Op-Ord" => Attribute::OperatorOrder(values[0].parse::<usize>().unwrap()),
            "Right-Assoc" => Attribute::RightAssociative,
            "Left-Assoc" => Attribute::LeftAssociative,
            "Atomic" => Attribute::Atomic,
            "ThreadLocal" => Attribute::ThreadLocal,
            "ThreadShared" => Attribute::ThreadShared,
            "ThreadMutable" => Attribute::ThreadMutable,
            "ThreadSpawn" => Attribute::ThreadSpawn,
            "Minimal" => Attribute::Minimal,
            "Default" => Attribute::Default,
            "Control" => Attribute::Control,
            "Final" => Attribute::Final,
            _ => panic!("Unknown attribute"),
        });
    
            
    let attribute = just(Token::Attribute).ignore_then(basic_attribute);

    attribute.repeated()
}

#[cfg(test)]
mod attribute_parser_tests {
    use super::*;
    use crate::parser::lexer::lexer;

    #[test]
    fn test_single_attribute() {
        let input = "@Atomic";

        let tokens = lexer(input);

        if tokens.is_err() {
            panic!("Lexer error: {:?}", tokens.err());
        }

        let tokens = tokens.unwrap();

        let result = attribute_parser().parse(tokens);

        if result.is_err() {
            panic!("Parser error: {:?}", result.err());
        }

        let result = result.unwrap();

        assert_eq!(result, vec![Attribute::Atomic], "Incorrect attribute");
        
    }

    #[test]
    fn test_single_attribute_with_value() {
        let input = "@Op-Ord 1";

        let tokens = lexer(input);

        if tokens.is_err() {
            panic!("Lexer error: {:?}", tokens.err());
        }

        let tokens = tokens.unwrap();

        let result = attribute_parser().parse(tokens);

        if result.is_err() {
            panic!("Parser error: {:?}", result.err());
        }

        let result = result.unwrap();

        assert_eq!(result, vec![Attribute::OperatorOrder(1)], "Incorrect attribute");
        
    }

    #[test]
    fn test_multiple_attributes() {
        let input = "@Atomic\n@Op-Ord 1";

        let tokens = lexer(input);

        if tokens.is_err() {
            panic!("Lexer error: {:?}", tokens.err());
        }

        let tokens = tokens.unwrap();

        let result = attribute_parser().parse(tokens);

        if result.is_err() {
            panic!("Parser error: {:?}", result.err());
        }

        let result = result.unwrap();

        assert_eq!(result, vec![Attribute::Atomic, Attribute::OperatorOrder(1)], "Incorrect attribute");
        
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    None,
    Left,
    Right,
}

lazy_static! {
    pub static ref Operator_Order: RwLock<HashMap<String, usize>> = RwLock::new(HashMap::new());
    pub static ref Operator_Associativity: RwLock<HashMap<String, Associativity>> = RwLock::new(HashMap::new());
}

pub fn add_operator_order(name: String, order: usize) {
    let mut map = Operator_Order.write().unwrap();
    map.insert(name, order);
}

pub fn get_operator_order(name: String) -> Option<usize> {
    let map = Operator_Order.read().unwrap();
    map.get(&name).cloned()
}

pub fn add_operator_associativity(name: String, associativity: Associativity) {
    let mut map = Operator_Associativity.write().unwrap();
    map.insert(name, associativity);
}

pub fn get_operator_associativity(name: String) -> Option<Associativity> {
    let map = Operator_Associativity.read().unwrap();
    map.get(&name).cloned()
}


pub fn infix_function_prototype_parser() -> impl Parser<Token, Result<Type, (String, Value)>, Error = Simple<Token>> {

    let parser_without_effects = attribute_parser()
        .then_ignore(just(Token::Function))
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
        .map(|(((attributes, name), args), return_type)| {
            let mut set_assoc = false;
            for attribute in attributes.iter() {
                match attribute {
                    Attribute::OperatorOrder(order) => add_operator_order(name.clone(), *order),
                    Attribute::RightAssociative => {
                        set_assoc = true;
                        add_operator_associativity(name.clone(), Associativity::Right)
                    },
                    Attribute::LeftAssociative => {
                        set_assoc = true;
                        add_operator_associativity(name.clone(), Associativity::Left)
                    },
                    _ => (),
                }
            }
            if !set_assoc {
                add_operator_associativity(name.clone(), Associativity::None)
            }

            Ok(Value::Function(attributes, args.iter().map(|x| (String::new(), Some(x.clone()))).collect(), Vec::new(), return_type, HashMap::new(), String::new()).get_type())
            
        });

    let parser_with_effects = attribute_parser()
        .then_ignore(just(Token::Function))
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
                .delimited_by(just(Token::ParenLeft), just(Token::ParenRight)))
        .then_ignore(just(Token::FunctionReturn))
        .then(type_parser())
        .map(|((((attributes, name), args), effects), return_type)| {
            let mut set_assoc = false;
            for attribute in attributes.iter() {
                match attribute {
                    Attribute::OperatorOrder(order) => add_operator_order(name.clone(), *order),
                    Attribute::RightAssociative => {
                        set_assoc = true;
                        add_operator_associativity(name.clone(), Associativity::Right)
                    },
                    Attribute::LeftAssociative => {
                        set_assoc = true;
                        add_operator_associativity(name.clone(), Associativity::Left)
                    },
                    _ => (),
                }
            }
            if !set_assoc {
                add_operator_associativity(name.clone(), Associativity::None)
            }

            Ok(Value::Function(attributes, args.iter().map(|x| (String::new(), Some(x.clone()))).collect(), effects, return_type, HashMap::new(), String::new()).get_type())
            
        });

    parser_with_effects.or(parser_without_effects)

}

fn function_argument_parser() -> impl Parser<Token, (String, Option<Type>), Error = Simple<Token>> {

    let typed_arg = filter_map(|span, token| match token {
        Token::Identifier(name) => Ok(name),
        _ => Err(Simple::custom(span, "Expected identifier".to_string())),
    }).then(type_statement_parser())
        .map(|(name, type_)| (name, Some(type_)));

    let untyped_arg = filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
    }).map(|name| (name, None));

    choice((typed_arg, untyped_arg))
}
    

/*pub fn infix_function_parser() -> impl Parser<Token, Result<Type, (String, Value)>, Error = Simple<Token>> {

    let parser_without_effects = attribute_parser()
        .then_ignore(just(Token::Function))
        .then_ignore(just(Token::ParenLeft))
        .then(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        }))
        .then_ignore(just(Token::ParenRight))
        .then(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        })).then(type_parser())
              .separated_by(just(Token::Comma))
              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
        .then_ignore(just(Token::FunctionReturn))
        .then(type_parser())
        .then(code_block_parser())
        .map(|((((attributes, name), args), return_type), code_block)| {
            let mut set_assoc = false;
            for attribute in attributes.iter() {
                match attribute {
                    Attribute::OperatorOrder(order) => add_operator_order(name.clone(), order),
                    Attribute::RightAssociative => {
                        set_assoc = true;
                        add_operator_associativity(name.clone(), Associativity::Right)
                    },
                    Attribute::LeftAssociative => {
                        set_assoc = true;
                        add_operator_associativity(name.clone(), Associativity::Left)
                    },
                    _ => (),
                }
            }
            if !set_assoc {
                add_operator_associativity(name.clone(), Associativity::None)
            }

            Err((name,Value::Function(attributes, args.iter().map(|x| (String::new(), Some(x.clone()))).collect(), Vec::new(), return_type, HashMap::new(), code_block)))
            
        });

    let parser_with_effects = attribute_parser()
        .then_ignore(just(Token::Function))
        .then_ignore(just(Token::ParenLeft))
        .then(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        }))
        .then_ignore(just(Token::ParenRight))
        .then(filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        })).then(type_parser())
              .separated_by(just(Token::Comma))
              .delimited_by(just(Token::ParenLeft), just(Token::ParenRight))
        .then(type_parser()
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::ParenLeft), just(Token::ParenRight)))
        .then_ignore(just(Token::FunctionReturn))
        .then(type_parser())
        .then(code_block_parser())
        .map(|((((attributes, name), args), effects), return_type), code_block| {
            let mut set_assoc = false;
            for attribute in attributes.iter() {
                match attribute {
                    Attribute::OperatorOrder(order) => add_operator_order(name.clone(), order),
                    Attribute::RightAssociative => {
                        set_assoc = true;
                        add_operator_associativity(name.clone(), Associativity::Right)
                    },
                    Attribute::LeftAssociative => {
                        set_assoc = true;
                        add_operator_associativity(name.clone(), Associativity::Left)
                    },
                    _ => (),
                }
            }
            if !set_assoc {
                add_operator_associativity(name.clone(), Associativity::None)
            }

            Err((name,Value::Function(attributes, args.iter().map(|x| (String::new(), Some(x.clone()))).collect(), effects, return_type, HashMap::new(), code_block)))
            
        });

    choice((parser_with_effects, parser_without_effects))

}*/





