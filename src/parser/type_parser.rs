use chumsky::prelude::*;

use crate::parser::symbols_parser::identifiers;

pub enum Type {
    WithTypeList {
        name: String,
        parameters: Vec<Type>,
    },
    Function {
        parameters: Vec<(Type)>,
        effects: Vec<Type>,
        return_type: Box<Type>,
    },
    SingleType(String),
    Tuple(Vec<Type>),
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


    let function = parse_function();

    choice((single_type, tuple, type_group, function))

}
