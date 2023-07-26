use chumsky::prelude::*;

use crate::parser::lexer::{Token, lexer};

pub enum Type {
    WithTypeList {
        name: String,
        parameters: Vec<Type>,
    },
    Function {
        parameters: Vec<(Option<String>, Type)>,
        effects: Vec<String>,
        return_type: Box<Type>,
    },
    SingleType(String),
    Tuple(Vec<Type>),
}


fn parse_identifier() -> impl Parser<Token, String, Error = Simple<Token>> {
    
    let identifier = just(Token::Identifier(_))
        .map(|x| match x {
            Token::Identifier(x) => x,
            _ => unreachable!(),
        });

    identifier
}


/*

/// This function parses a single generic type list, e.g. `<a, b, c>`
/// This should also parse this <List <a>, b, c>
fn parse_type_list() -> impl Parser<char, Vec<Type>, Error = Simple<char>> {

    let type_item = parse_type().padded()
        .then(symbols().just::<char,Token, Simple<char>>(Token::Comma).padded())
        .or(symbols().just(Token::ParenRight).padded().ignore());
    
    let generic_type_list = just("<").padded()
        .then(list.repeated())
        .then(just(">")).padded()
        .map(|((_, types), _)| types.into_iter().map(|x| Type::SingleType(x)).collect());

    generic_type_list
}


/// This should parse something like this List <a> or List<Int>
fn parse_type_with_list() -> impl Parser<char, Type, Error = Simple<char>> {

    let type_with_generic = identifiers().map(|x| x.to_string()).padded()
        .then(parse_type_list()).padded()
        .map(|(name, parameters)| Type::WithTypeList { name, parameters });

    type_with_generic
}



/// This function parses one of the following:
/// a
/// fn(a, b, c) exn -> d
/// (a, b, c)
/// Eq <a>
/// Eq <List <a>>
/// List <a>
fn parse_type() -> impl Parser<char, Type, Error = Simple<char>> {

    // Parses a, or a
    let tuple_body = parse_type().padded()
        .then(symbols().just(Token::Comma).padded())
        .or(symbols().just(Token::ParenRight).padded().ignore());

    // Parses ()
    let empty_tuple = symbols().just(Token::ParenLeft).padded()
        .then(symbols().just(Token::ParenRight).padded())
        .map(|_| Type::Tuple(vec![]));

    // Parses (a, b, c) or (a)
    let filled_tuple = symbols().just(Token::ParenLeft).padded()
        .then(tuple_body.repeated())
        .then(symbols().just(Token::ParenRight).padded())
        .map(|((_, types), _)| Type::Tuple(types));

    // Parses (a, b, c) or (a) or ()
    let tuple = choice((empty_tuple, filled_tuple));

    // Parses a
    let single_type = identifiers().map(|x| x.to_string()).padded()
        .map(|x| Type::SingleType(x));

    let type_with_sub_types = identifiers().map(|x| x.to_string()).padded()
        .then(parse_type_list()).padded()
        .map(|(name, parameters)| Type::WithTypeList { name, parameters });

    let function_type = keywords().just(Token::Function).padded()
        .then(tuple)
        .then(parse_effects().padded())
        .then(symbols().just(Token::FunctionReturn).padded())
        .then(parse_type().padded())
        .map(|((((_, parameters), _), effects), return_type)| Type::Function { parameters, effects, return_type });


    let types = choice((type_with_sub_types, function_type, tuple, single_type));

    types
}



fn parse_effects() -> impl Parser<char, Vec<String>, Error = Simple<char>> {

    
    let effect = identifiers().map(|x| x.to_string()).padded();

    let effect_list = effect
        .then(symbols().just(Token::Comma).padded())
        .or(just("->").padded().ignore())
        .repeated()
        .map(|(effects, _)| effects);

    effects
}*/
