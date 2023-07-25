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
        
        let class = text::keyword::<_, _, Simple<char>>("class").map(|_| Token::Class).padded();
        let instance = text::keyword::<_, _, Simple<char>>("instance").map(|_| Token::Instance).padded();
        let default = text::keyword::<_, _, Simple<char>>("default").map(|_| Token::Default).padded();
        let sum = text::keyword::<_, _, Simple<char>>("sum").map(|_| Token::Sum).padded();
        let product = text::keyword::<_, _, Simple<char>>("product").map(|_| Token::Product).padded();
        let type_ = text::keyword::<_, _, Simple<char>>("type").map(|_| Token::Type).padded();
        let function = text::keyword::<_, _, Simple<char>>("fn").map(|_| Token::Function).padded();
        let match_ = text::keyword::<_, _, Simple<char>>("match").map(|_| Token::Match).padded();
        let while_ = text::keyword::<_, _, Simple<char>>("while").map(|_| Token::While).padded();
        let elwhile = text::keyword::<_, _, Simple<char>>("elwhile").map(|_| Token::ElWhile).padded();
        let for_ = text::keyword::<_, _, Simple<char>>("for").map(|_| Token::For).padded();
        let loop_ = text::keyword::<_, _, Simple<char>>("loop").map(|_| Token::Loop).padded();
        let if_ = text::keyword::<_, _, Simple<char>>("if").map(|_| Token::If).padded();
        let elif = text::keyword::<_, _, Simple<char>>("elif").map(|_| Token::Elif).padded();
        let else_ = text::keyword::<_, _, Simple<char>>("else").map(|_| Token::Else).padded();
        let continue_ = text::keyword::<_, _, Simple<char>>("continue").map(|_| Token::Continue).padded();
        let break_ = text::keyword::<_, _, Simple<char>>("break").map(|_| Token::Break).padded();
        let in_ = text::keyword::<_, _, Simple<char>>("in").map(|_| Token::In).padded();
        let typeis = text::keyword::<_, _, Simple<char>>("typeis").map(|_| Token::Typeis).padded();
        let effect = text::keyword::<_, _, Simple<char>>("effect").map(|_| Token::Effect).padded();
        let with = text::keyword::<_, _, Simple<char>>("with").map(|_| Token::With).padded();
        let return_ = text::keyword::<_, _, Simple<char>>("return").map(|_| Token::Return).padded();
        let mod_ = text::keyword::<_, _, Simple<char>>("mod").map(|_| Token::Mod).padded();
        let import = text::keyword::<_, _, Simple<char>>("import").map(|_| Token::Import).padded();

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
        let result = keywords().parse(" instance");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing instance");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Instance, "Error parsing instance");
    }

    #[test]
    fn test_return() {
        let result = keywords().parse("return ");

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
            just(":").to(Token::Colon).padded(),
            just(".").to(Token::Period),
            just(":=").to(Token::MutableAssignment).padded(),
            just("@").to(Token::Attribute).padded(),
            just("_").to(Token::WildCard).padded(),
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
        let result = operators().parse(": ");

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

    #[test]
    fn test_period_fail() {
        let result = operators().parse(" .");

        if result.is_ok() {
            eprintln!("{:?}", result);
            assert!(false, "Parser should have failed");
        }
    }
}


fn symbols() -> impl Parser<char, Token, Error = Simple<char>> {

    let symbol = recursive(|sym| {
        choice((
            just("[").to(Token::BracketLeft).padded(),
            just("]").to(Token::BracketRight).padded(),
            just("(").to(Token::ParenLeft).padded(),
            just(")").to(Token::ParenRight).padded(),
            just("{").to(Token::CurlyLeft).padded(),
            just("}").to(Token::CurlyRight).padded(),
            just(",").to(Token::Comma).padded(),
            just(";").to(Token::Semicolon).padded(),
            just("->").to(Token::FunctionReturn).padded(),
            just("=>").to(Token::MatchArm).padded(),
            just("::").to(Token::Namespace),
            ))
    });

    symbol
}


#[cfg(test)]
mod symbol_tests {
    use super::*;
    use chumsky::prelude::*;

    #[test]
    fn test_bracket() {
        let result = symbols().parse("[");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing bracket");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::BracketLeft, "Token not bracket");
    }

    #[test]
    fn test_namespace_fail() {
        let result = symbols().parse(" ::");

        if result.is_ok() {
            eprintln!("{:?}", result);
            assert!(false, "Parser should have failed");
        }
    }

}

/*fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {



    


}*/




