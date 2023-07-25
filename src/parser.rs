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
            just(":=").to(Token::MutableAssignment).padded(),
            just("=").to(Token::Assignment).padded(),
            just(":").to(Token::Colon).padded(),
            just(".").to(Token::Period),
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
    fn test_mut_assignment() {
        let result = operators().parse(":=");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing mutable assignment");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::MutableAssignment, "Token not mutable assignment");
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


fn numbers() -> impl Parser<char, Token, Error = Simple<char>> {

    let dec = text::int(10).map(|i: String| i);
    let hex = text::int(16).map(|i: String| i);
    let oct = text::int(8).map(|i: String| i);
    let bin = text::int(2).map(|i: String| i);

    let sign = choice((
        just("+").to("+"),
        just("-").to("-"),
        just("").to(""),
    ));

    let hex_prefix = choice((
        just("0x").to("0x"),
        just("0X").to("0x"),
    ));

    let oct_prefix = choice((
        just("0o").to("0o"),
        just("0O").to("0o"),
    ));

    let bin_prefix = choice((
        just("0b").to("0b"),
        just("0B").to("0b"),
    ));

    let int_suffix = choice((
        just("i").to("i"),
        just("I").to("i"),
        just("u").to("u"),
        just("U").to("u"),
        just("f").to("f"),//converts to float
        just("F").to("f"),//converts to float
        just("").to(""),
    ));

    let dec_point = just(".").to(".");
    let exp_mark = choice((just("e").to("e"), just("E").to("e")));

    let dec_int = sign
        .then(dec.map(|i| i))
        .then(int_suffix.map(|s| s))
        .map(|((s, i), suf)| Token::Number(s.to_owned() + &i + &suf));

    let hex_int = sign
        .then(hex_prefix.map(|p| p))
        .then(hex.map(|i| i))
        .then(int_suffix.map(|s| s))
        .map(|(((s, p), i), suf)| Token::Number(s.to_owned() + &p + &i + &suf));

    let oct_int = sign
        .then(oct_prefix.map(|p| p))
        .then(oct.map(|i| i))
        .then(int_suffix.map(|s| s))
        .map(|(((s, p), i), suf)| Token::Number(s.to_owned() + &p + &i + &suf));

    let bin_int = sign
        .then(bin_prefix.map(|p| p))
        .then(bin.map(|i| i))
        .then(int_suffix.map(|s| s))
        .map(|(((s, p), i), suf)| Token::Number(s.to_owned() + &p + &i + &suf));

    let integer = choice((hex_int, oct_int, bin_int, dec_int));
    
    let exp = exp_mark
        .then(sign.map(|s| s))
        .then(dec.map(|i| i))
        .map(|((e, s), i)| e.to_owned() + &s + &i);

    let float_exp = sign
        .then(dec.map(|i| i))
        .then(dec_point.map(|p| p.to_string()))
        .then(dec.map(|i| i))
        .then(exp.map(|e| e))
        .map(|((((s, i), p), i2), e)| Token::Number(s.to_owned() + &i + &p + &i2 + &e));


    let float_wo_exp = sign
        .then(dec.map(|i| i))
        .then(dec_point.map(|p| p.to_string()))
        .then(dec.map(|i| i))
        .map(|((((s, i), p), i2))| Token::Number(s.to_owned() + &i + &p + &i2));

    let float = choice((float_exp, float_wo_exp));
        

    let number = choice((float, integer)).padded();

        
    number
}


#[cfg(test)]
mod number_tests {
    use super::*;

    #[test]
    fn test_dec() {
        let result = numbers().parse("123");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing decimal");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("123".to_string()), "Token not decimal");
    }

    #[test]
    fn test_hex() {
        let result = numbers().parse("0x1aF");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing hex");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("0x1aF".to_string()), "Token not hex");
    }

    #[test]
    fn test_octal() {
        let result = numbers().parse("0o123");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing octal");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("0o123".to_string()), "Token not octal");
    }

    #[test]
    fn test_binary() {
        let result = numbers().parse("0b101");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing binary");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("0b101".to_string()), "Token not binary");
    }

    #[test]
    fn test_float() {
        let result = numbers().parse("123.456");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing float");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("123.456".to_string()), "Token not float");
    }

    #[test]
    fn test_float_exp() {
        let result = numbers().parse("123.456e-10");
        
        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing float with exp");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("123.456e-10".to_string()), "Token not float with exp");
    }

    #[test]
    fn test_padded_number() {
        let result = numbers().parse(" 123.456e-10 ");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing padded number");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("123.456e-10".to_string()), "Token not padded number");
    }

    #[test]
    fn test_integer_suffix() {
        let result = numbers().parse("123i");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing integer suffix");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("123i".to_string()), "Token not integer suffix");
    }
}

/*fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {



    


}*/




