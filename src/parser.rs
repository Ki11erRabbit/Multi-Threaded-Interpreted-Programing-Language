use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::prelude::*;

use std::ops::Range;


//TODO: Change String to &str
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    WhiteSpace,
    Number(String),
    String(String),
    Char(String),
    Identifier(String),
    Operator(String),
    Assignment, // =
    MutableAssignment, // :=
    BracketLeft, // [
    BracketRight, // ]
    ParenLeft, // (
    ParenRight, // )
    CurlyLeft, // {
    CurlyRight, // }
    Attribute, // @
    Comma, // ,
    Semicolon, // ;
    Colon, // :
    Namespace, // ::
    Period, // .
    WildCard, // _
    Comment,
    MatchArm, // =>
    FunctionReturn, // ->
    Class,
    Instance,
    Default,
    Sum,
    Product,
    Type,
    Function, // fn
    Match,
    While,
    ElWhile,
    For,
    Loop,
    If,
    Elif,
    Else,
    Continue,
    Break,
    In,
    Typeis,
    Effect,
    With,
    Return,
    Mod,
    Import,
}


fn keywords() -> impl Parser<char, Token, Error = Simple<char>> {

    let keyword = recursive(|kwd| {
        
        let class = text::keyword::<_, _, Simple<char>>("class").map(|_| Token::Class);
        let instance = text::keyword::<_, _, Simple<char>>("instance").map(|_| Token::Instance);
        let default = text::keyword::<_, _, Simple<char>>("default").map(|_| Token::Default);
        let sum = text::keyword::<_, _, Simple<char>>("sum").map(|_| Token::Sum);
        let product = text::keyword::<_, _, Simple<char>>("product").map(|_| Token::Product);
        let type_ = text::keyword::<_, _, Simple<char>>("type").map(|_| Token::Type);
        let function = text::keyword::<_, _, Simple<char>>("fn").map(|_| Token::Function);
        let match_ = text::keyword::<_, _, Simple<char>>("match").map(|_| Token::Match);
        let while_ = text::keyword::<_, _, Simple<char>>("while").map(|_| Token::While);
        let elwhile = text::keyword::<_, _, Simple<char>>("elwhile").map(|_| Token::ElWhile);
        let for_ = text::keyword::<_, _, Simple<char>>("for").map(|_| Token::For);
        let loop_ = text::keyword::<_, _, Simple<char>>("loop").map(|_| Token::Loop);
        let if_ = text::keyword::<_, _, Simple<char>>("if").map(|_| Token::If);
        let elif = text::keyword::<_, _, Simple<char>>("elif").map(|_| Token::Elif);
        let else_ = text::keyword::<_, _, Simple<char>>("else").map(|_| Token::Else);
        let continue_ = text::keyword::<_, _, Simple<char>>("continue").map(|_| Token::Continue);
        let break_ = text::keyword::<_, _, Simple<char>>("break").map(|_| Token::Break);
        let in_ = text::keyword::<_, _, Simple<char>>("in").map(|_| Token::In);
        let typeis = text::keyword::<_, _, Simple<char>>("typeis").map(|_| Token::Typeis);
        let effect = text::keyword::<_, _, Simple<char>>("effect").map(|_| Token::Effect);
        let with = text::keyword::<_, _, Simple<char>>("with").map(|_| Token::With);
        let return_ = text::keyword::<_, _, Simple<char>>("return").map(|_| Token::Return);
        let mod_ = text::keyword::<_, _, Simple<char>>("mod").map(|_| Token::Mod);
        let import = text::keyword::<_, _, Simple<char>>("import").map(|_| Token::Import);

        class
            .or(instance)
            .or(default)
            .or(sum)
            .or(product)
            .or(type_)
            .or(function)
            .or(match_)
            .or(while_)
            .or(elwhile)
            .or(for_)
            .or(loop_)
            .or(if_)
            .or(elif)
            .or(else_)
            .or(continue_)
            .or(break_)
            .or(in_)
            .or(typeis)
            .or(effect)
            .or(with)
            .or(return_)
            .or(mod_)
            .or(import)
    });

    
    keyword
}

#[cfg(test)]
mod keywords_tests {
    use super::*;
    use chumsky::prelude::*;

    #[test]
    fn test_class() {
        let result = keywords().parse("class");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing class");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Class, "Error parsing class");
    }
    #[test]
    fn test_instance() {
        let result = keywords().parse("instance");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing instance");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Instance, "Error parsing instance");
    }

    #[test]
    fn test_return() {
        let result = keywords().parse("return");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing return");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Return, "Error parsing return");
    }


}


fn operators() -> impl Parser<char, Token, Error = Simple<char>> {

    let operator = recursive(|op| {
        choice((
            just("=").to(Token::Assignment).padded(),
            just(":").to(Token::Colon),
            just(".").to(Token::Period),
            just(":=").to(Token::MutableAssignment),
            just("@").to(Token::Attribute),
            just("_").to(Token::WildCard),
            ))
    });

    operator
}



#[cfg(test)]
mod operator_tests {
    use super::*;
    use chumsky::prelude::*;

    #[test]
    fn test_assignment() {
        let result = operators().parse("=");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing assignment");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Assignment, "Token not assignment");
    }

    #[test]
    fn test_colon() {
        let result = operators().parse(":");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing colon");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Colon, "Token not colon");
    }

    #[test]
    fn test_period() {
        let result = operators().parse(".");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing period");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Period, "Token not period");
    }


}

/*fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {



    


}*/




