use chumsky::prelude::*;

use crate::parser::symbols_parser::{identifiers, keyword_function,symbol_paren_left, symbol_paren_right,symbol_function_arrow};

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
    SingleType(String),
    Tuple(Vec<Type>),
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::SingleType(name) => write!(f, "{}", name),
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

/*
/// This function parses something like this (List a) or (List (a, b, c))
/// Or (Hashmap a b)
fn parse_type_group() -> impl Parser<char, Type, Error = Simple<char>> {

    let identifier = identifiers();

    /*let type_group = just('(')
        .ignore_then(identifier).padded()
        .then(parse_type().padded().repeated())
        .then_ignore(just(')'))
        .map(|(name, parameters)| Type::WithTypeList { name, parameters });*/

    let type_group = identifier.padded()
        .then(parse_type().padded().repeated())
        .map(|(name, parameters)| Type::TypeList { name, parameters });

    type_group

}*/

/// This function parses something like this (a, b, c) or ()
fn parse_tuple() -> impl Parser<char, Type, Error = Simple<char>> {

    let tuple_body = parse_type().padded()
        .then_ignore(one_of(",)").padded());


    let empty_tuple = just('(')
        .ignore_then(just(')').padded())
        .map(|_| Type::Tuple(Vec::new()));

    let filled_tuple = just('(')
        .ignore_then(tuple_body.repeated()).padded()
        .then_ignore(just(')'))
        .map(|types| Type::Tuple(types));

    let tuple = choice((empty_tuple, filled_tuple));

    tuple
}

/// This function parses something like this:
/// fn(a, b, c) exn -> Int
fn parse_function() -> impl Parser<char, Type, Error = Simple<char>> {

    let parameter = parse_type().padded()
        .then_ignore(one_of(",)").padded());

    let effects = parse_type().padded()
        .then_ignore(choice((just(","), just("->"))).padded());

    let function_with_effects = text::keyword("fn")
        .ignore_then(just('('))
        .ignore_then(parameter.repeated())
        .then_ignore(just(')'))
        .then(effects.repeated()).padded()
        .then_ignore(just("->"))
        .then(parse_type().padded())
        .map(|((parameters, effects), return_type)| Type::Function { parameters, effects, return_type: Box::new(return_type) });


    let parameter = parse_type().padded()
        .then_ignore(one_of(",)").padded());

    let function_without_effects = text::keyword("fn")
        .ignore_then(just('('))
        .ignore_then(parameter.repeated())
        .then_ignore(just(')'))
        .then_ignore(just("->"))
        .then(parse_type().padded())
        .map(|(parameters, return_type)| Type::Function { parameters, effects: Vec::new(), return_type: Box::new(return_type) });

    choice((function_with_effects, function_without_effects))
}


/// This function parses types like:
/// (a, b, c)
/// (List a)
/// a
/// Int
/// fn(a, b, c) exn -> Int
fn parse_type() -> impl Parser<char, Type, Error = Simple<char>> {

    let single_type = identifiers()
        .map(|name| Type::SingleType(name));

    /*let type_group = parse_type_group()
        .map(|type_group| type_group);*/
    /*let tuple = parse_tuple();


    let tuple_or_type_group = recursive(|t| t
                                        .separated_by(just(',').padded())
                                        .delimited_by(just('(').padded(), just(')').padded())
                                        .then(tuple)
                                        .map(|(tuple, _)| Type::Tuple(tuple))
                                        .or(type_group)
                                        .map(|type_group| type_group));


    let function = parse_function();*/

    /*let single_or_group:Recursive<char, Type, Simple<char>> = recursive(|sog: chumsky::recursive::Recursive<'_, char, Type, Simple<char>>| sog
                                    .separated_by(just(' '))
                                    .delimited_by(just('(').padded(), just(')').padded())
                                    .map(|vec| Type::TypeList { name: Box::new(vec[0].clone()), parameters: vec[1..].to_vec() })
                                                                        .or(identifiers().map(Type::SingleType))
);*/

    let tuple_or_single = recursive(|tuple| tuple
                                    .separated_by(just(',').padded())
                                    .delimited_by(just('(').padded(), just(')').padded())
                                    .map(|tuple| Type::Tuple(tuple))
                                    .or(single_type)
    );

    let single_or_group:Recursive<char, Type, Simple<char>> = recursive(|sog: chumsky::recursive::Recursive<'_, char, Type, Simple<char>>| sog
                                    .separated_by(just(' '))
                                    .delimited_by(just('(').padded(), just(')').padded())
                                    .map(|vec| Type::TypeList { name: Box::new(vec[0].clone()), parameters: vec[1..].to_vec() })
                                                                        .or(tuple_or_single)
    );

    let multiple_effects = single_or_group.clone()
        .separated_by(just(',').padded());
    let effect = single_or_group.clone().map(|eff| vec![eff]);
    let no_effects = just("->").padded().rewind().map(|_| Vec::new());

    let effects = choice((multiple_effects, effect, no_effects));
    
    
    /*let effects: Recursive<char, Vec<Type>, Simple<char>> = recursive(|eff: chumsky::recursive::Recursive<'_, char, Vec<Type>, Simple<char>>| eff
                            .separated_by(just(',').padded())
                            .map(|effects| effects[0])
                            .or(single_or_group.map(|single_or_group| vec![vec![single_or_group]])));*/
        

    let function_args = single_or_group.clone()
        .separated_by(just(',').padded());

    let function = keyword_function().padded()
        .ignore_then(function_args)
        .then(effects)
        .then_ignore(symbol_function_arrow().padded())
        .then(single_or_group.clone())
        .map(|((parameters, effects), return_type)| Type::Function { parameters, effects, return_type: Box::new(return_type) });



    single_or_group.recover_with(skip_parser(function))
}


#[cfg(test)]
mod type_parse_tests {
    use super::*;

    #[test]
    fn test_simple_type() {
        let result = parse_type().parse("Int");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failure to parse simple type: Int");
        }

        let int = result.unwrap();
        
        assert_eq!(int, Type::SingleType("Int".to_string()));

    }

    #[test]
    fn test_type_list() {
        let result = parse_type().parse("(List String)");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failure to parse mixed type: (List String)");
        }

        let list_string = result.unwrap();

        assert_eq!(list_string, Type::TypeList { name: Box::new(Type::SingleType("List".to_string())), parameters: vec![Type::SingleType("String".to_string())] });
    }

    #[test]
    fn test_nested_type_list() {
        let result = parse_type().parse("(List (List String))");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failure to parse mixed type: (List (List String))");
        }

        let list_list_string = result.unwrap();

        assert_eq!(list_list_string, Type::TypeList { name: Box::new(Type::SingleType("List".to_string())), parameters: vec![ Type::TypeList { name : Box::new(Type::SingleType("List".to_string())), parameters: vec![ Type::SingleType("String".to_string())]}]}, "Nested Type List doesn't match");
    }


    #[test]
    fn test_long_type_list() {
        
        let result = parse_type().parse("(List String a b c)");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failure to parse mixed type: (List String a b c)");
        }

        let list_string = result.unwrap();

        assert_eq!(list_string, Type::TypeList { name: Box::new(Type::SingleType("List".to_string())), parameters: vec![Type::SingleType("String".to_string()), Type::SingleType("a".to_string()), Type::SingleType("b".to_string()), Type::SingleType("c".to_string())] });
        
    }

    #[test]
    fn test_tuple() {
        let result = parse_type().parse("(Int, String)");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failure to parse mixed type: (Int, String)");
        }

        let tuple = result.unwrap();

        assert_eq!(tuple, Type::Tuple(vec![Type::SingleType("Int".to_string()), Type::SingleType("String".to_string())]));
    }

    #[test]
    fn test_nested_tuples() {
        let result = parse_type().parse("(Int, (String, Int))");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failure to parse mixed type: (Int, (String, Int))");
        }

        let tuple = result.unwrap();

        assert_eq!(tuple, Type::Tuple(vec![Type::SingleType("Int".to_string()), Type::Tuple(vec![Type::SingleType("String".to_string()), Type::SingleType("Int".to_string())])]));
    }

    #[test]
    fn test_tuple_in_type_list() {
        let result = parse_type().parse("(List (Int, String))");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failure to parse mixed type: (List (Int, String))");
        }

        let tuple = result.unwrap();

        assert_eq!(tuple, Type::TypeList { name: Box::new(Type::SingleType("List".to_string())), parameters: vec![Type::Tuple(vec![Type::SingleType("Int".to_string()), Type::SingleType("String".to_string())])]} );
    }

    #[test]
    fn test_simple_function_type() {
        let result = parse_type().parse("fn (Int, String) -> Int");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Failure to parse mixed type: fn (Int, String) -> Int");
        }

        let function = result.unwrap();

        assert_eq!(function, Type::Function { parameters: vec![Type::Tuple(vec![Type::SingleType("Int".to_string()), Type::SingleType("String".to_string())])], effects: Vec::new(), return_type: Box::new(Type::SingleType("Int".to_string())) });
    }

        
    

}
