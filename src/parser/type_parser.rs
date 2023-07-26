use chumsky::prelude::*;

use crate::parser::symbols_parser::identifiers;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    WithTypeList {
        name: String,
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
            Type::WithTypeList{name, parameters} => {
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


/// This function parses something like this (List a) or (List (a, b, c))
/// Or (Hashmap a b)
fn parse_type_group() -> impl Parser<char, Type, Error = Simple<char>> {

    let identifier = identifiers();

    let type_group = just('(')
        .ignore_then(identifier).padded()
        .then(parse_type().padded().repeated())
        .then_ignore(just(')'))
        .map(|(name, parameters)| Type::WithTypeList { name, parameters });


    type_group

}

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

    let tuple = parse_tuple();

    let type_group = parse_type_group();

    let tuple_or_type_group = recursive(|t| t
                                        .separated_by(just(',').padded())
                                        .delimited_by(just('(').padded(), just(')').padded())
                                        .then(tuple)
                                        .map(|(tuple, _)| Type::Tuple(tuple))
                                        .or(type_group)
                                        .map(|type_group| type_group));


    let function = parse_function();

    let tree = recursive(|tree| tree
                         .separated_by(one_of(", ").padded())
                         .delimited_by(just('(').padded(), just(')').padded())
                         .then(tuple_or_type_group
                            .map(|type_group| vec![type_group]))
                         .or(single_type
                            .map(|single_type| vec![single_type])));

    tree
}


#[cfg(test)]
mod type_parse_tests {
    use super::*;

    #[test]
    fn test_simple_type() {
        let result = parse_type().parse("Int");

        if result.is_err() {
            println!("{:?}", result);
            assert!(false, "Failure to parse simple type: Int");
        }

        let int = result.unwrap();
        
        assert_eq!(int, Type::SingleType("Int".to_string()));

    }


}
