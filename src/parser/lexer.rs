use chumsky::prelude::*;

use std::fmt;


//TODO: Change String to &str
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    WhiteSpace(String),
    Number(String),
    String(String),
    Char(char),
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
    Comment(String),
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::WhiteSpace(s) => write!(f, "{}", s),
            Token::Number(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "{}", s),
            Token::Char(c) => write!(f, "{}", c),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Operator(s) => write!(f, "{}", s),
            Token::Assignment => write!(f, "="),
            Token::MutableAssignment => write!(f, ":="),
            Token::BracketLeft => write!(f, "["),
            Token::BracketRight => write!(f, "]"),
            Token::ParenLeft => write!(f, "("),
            Token::ParenRight => write!(f, ")"),
            Token::CurlyLeft => write!(f, "{{"),
            Token::CurlyRight => write!(f, "}}"),
            Token::Attribute => write!(f, "@"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Namespace => write!(f, "::"),
            Token::Period => write!(f, "."),
            Token::WildCard => write!(f, "_"),
            Token::Comment(s) => write!(f, "{}", s),
            Token::MatchArm => write!(f, "=>"),
            Token::FunctionReturn => write!(f, "->"),
            Token::Class => write!(f, "class"),
            Token::Instance => write!(f, "instance"),
            Token::Default => write!(f, "default"),
            Token::Sum => write!(f, "sum"),
            Token::Product => write!(f, "product"),
            Token::Type => write!(f, "type"),
            Token::Function => write!(f, "fn"),
            Token::Match => write!(f, "match"),
            Token::While => write!(f, "while"),
            Token::ElWhile => write!(f, "elwhile"),
            Token::For => write!(f, "for"),
            Token::Loop => write!(f, "loop"),
            Token::If => write!(f, "if"),
            Token::Elif => write!(f, "elif"),
            Token::Else => write!(f, "else"),
            Token::Continue => write!(f, "continue"),
            Token::Break => write!(f, "break"),
            Token::In => write!(f, "in"),
            Token::Typeis => write!(f, "typeis"),
            Token::Effect => write!(f, "effect"),
            Token::With => write!(f, "with"),
            Token::Return => write!(f, "return"),
            Token::Mod => write!(f, "mod"),
            Token::Import => write!(f, "import"),
        }
    }
            
}


pub fn keywords() -> impl Parser<char, Token, Error = Simple<char>> {

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

        choice((class
           ,instance
           ,default
           ,sum
           ,product
           ,type_
           ,function
           ,match_
           ,while_
           ,elwhile
           ,for_
           ,loop_
           ,if_
           ,elif
           ,else_
           ,continue_
           ,break_
           ,in_
           ,typeis
           ,effect
           ,with
           ,return_
           ,mod_
           ,import))
    });

    
    keyword
}

#[cfg(test)]
mod keywords_tests {
    use super::*;

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


pub fn operators() -> impl Parser<char, Token, Error = Simple<char>> {

    let operator = recursive(|op| {
        choice((
            just(":=").to(Token::MutableAssignment),
            just("=").to(Token::Assignment),
            just(":").to(Token::Colon),
            just(".").to(Token::Period),
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


pub fn symbols() -> impl Parser<char, Token, Error = Simple<char>> {

    let symbol = recursive(|sym| {
        choice((
            just("[").to(Token::BracketLeft),
            just("]").to(Token::BracketRight),
            just("(").to(Token::ParenLeft),
            just(")").to(Token::ParenRight),
            just("{").to(Token::CurlyLeft),
            just("}").to(Token::CurlyRight),
            just(",").to(Token::Comma),
            just(";").to(Token::Semicolon),
            just("->").to(Token::FunctionReturn),
            just("=>").to(Token::MatchArm),
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
        

    let number = choice((float, integer));

        
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

fn strings() -> impl Parser<char, Token, Error = Simple<char>> {

    let escape = just::<char, char, Simple<char>>('\\')
        .then(one_of("\"\\nrt "))
        .map(|(_, c)| match c {
            '"' => '"',
            '\\' => '\\',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            ' ' => ' ',
            _ => unreachable!()
        });

    let string_char = none_of("\"\\")
        .or(escape);

    let string = just('"')
        .ignore_then(string_char.repeated())
        .then_ignore(just('"'))
        .then_ignore(end())
        .map(|s| Token::String(s.iter().collect()));
    

    string
}

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn test_basic_string() {
        let result = strings().parse("\"Hello World\"");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing basic string");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::String("Hello World".to_string()), "Token not basic string");
    }

    #[test]
    fn test_escaped_string() {
        let result = strings().parse("\"Hello \\\"World\\\"\"");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing escaped string");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::String("Hello \"World\"".to_string()), "Token not escaped string");
    }

    #[test]
    fn test_escapes_string() {
        let result = strings().parse("\"Hello \\\\ \\n \\r \\t\"");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing escapes string");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::String("Hello \\ \n \r \t".to_string()), "Token not escapes string");
    }

}

fn chars() -> impl Parser<char, Token, Error = Simple<char>> {
    
    let escape = just::<char, char, Simple<char>>('\\')
        .then(one_of("\'\\nrt "))
        .map(|(_, c)| match c {
            '\'' => '\'',
            '\\' => '\\',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            ' ' => ' ',
            _ => unreachable!()
        });

    let possible_char = none_of("'\\")
        .or(escape);

    let char_ = just('\'')
        .ignore_then(possible_char)
        .then_ignore(just('\''))
        .then_ignore(end())
        .map(|s| Token::Char(s));
    

    char_
}

#[cfg(test)]
mod char_tests {
    use super::*;

    #[test]
    fn test_basic_char() {
        let result = chars().parse("'a'");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing basic char");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Char('a'), "Token not basic char");
    }

    #[test]
    fn test_escaped_char() {
        let result = chars().parse("'\\''");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing escaped char");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Char('\''), "Token not escaped char");
    }
}


pub fn literals() -> impl Parser<char, Token, Error = Simple<char>> {
    strings()
        .or(chars())
        .or(numbers())
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    #[test]
    fn test_string() {
        let result = literals().parse("\"Hello World\"");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing string");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::String("Hello World".to_string()), "Token not string");
    }

    #[test]
    fn test_char() {
        let result = literals().parse("'a'");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing char");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Char('a'), "Token not char");
    }

    #[test]
    fn test_number() {
        let result = literals().parse("123.456e-10");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing number");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Number("123.456e-10".to_string()), "Token not number");
    }
}


pub fn identifiers() -> impl Parser<char, Token, Error = Simple<char>> {

    let cant_start_with = none_of::<char, &str,Simple<char>>("0123456789 \n\t\r'\"\\,()[]{}@;:");
    let cant_contain = none_of(" \n\t\r'\"\\,()[]{}@;:");

    let cant_be = choice((
        just(":="),
        just("::"),
        just("->"),
        just("=>"),
        just("."),
        just("="),
        ));

    let special_identifiers = choice((
        just("get[]").map(|s| s.to_string()),
        just("set[]").map(|s| s.to_string()),
        just("con[]").map(|s| s.to_string()),
        just("con{}").map(|s| s.to_string()),
        just("add:").map(|s| s.to_string()),
        just("remove:").map(|s| s.to_string()),
        just("..").map(|s| s.to_string()),
        just("...").map(|s| s.to_string()),
        just("..=").map(|s| s.to_string()),
    ));


    let normal = cant_start_with
        .then(cant_contain.repeated())
        .map(|(c, s)| format!("{}{}", c, s.iter().collect::<String>()));

    let basic_identifier = 
        cant_be.not()
            .then(normal)
            .map(|(c,s)| {c.to_string() + &s});

    let identifier = choice((
        special_identifiers,
        basic_identifier,
    ))
    .map(|s| Token::Identifier(s));
        
        
    

    identifier
}

#[cfg(test)]
mod identifier_tests {
    use super::*;

    #[test]
    fn test_simple_identifier() {
        let result = identifiers().parse("hello");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing simple identifier");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Identifier("hello".to_string()), "Token not simple identifier");
    }

    #[test]
    fn test_complex_identifier() {
        let result = identifiers().parse("helloWorld123");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing complex identifier");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Identifier("helloWorld123".to_string()), "Token not complex identifier");
    }

    #[test]
    fn test_special_identifier() {
        let result = identifiers().parse("get[]");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing special identifier");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Identifier("get[]".to_string()), "Token not special identifier");
    }

    #[test]
    fn test_foreign_identifiers() {
        let result = identifiers().parse("かさ");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing foreign identifier");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Identifier("かさ".to_string()), "Token not foreign identifier");
    }

    #[test]
    fn test_cant_start_with() {
        let result = identifiers().parse("123");

        if result.is_ok() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing identifier starting with number");
        }
    }

    #[test]
    fn test_cant_contain() {
        let result = identifiers().parse("hello world");

        if result.is_ok() {
            eprintln!("{:?}", result);
            assert_ne!(result.unwrap(),Token::Identifier("hello world".to_string()), "Error parsing identifier containing space");
        }
    }

    #[test]
    fn test_symbols() {
        let result = identifiers().parse("++");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing symbol");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Identifier("++".to_string()), "Token not symbol");
    }

    #[test]
    fn test_edge_case() {
        let result = identifiers().parse("..");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing edge case");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Identifier("..".to_string()), "Token not edge case");
    }

    #[test]
    fn test_weird_edge_case() {
        let result = identifiers().parse("a = 1");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing weird edge case");
        }
    }

}

pub fn comments() -> impl Parser<char, Token, Error = Simple<char>> {
    let line_comment = just("//")
        .then(none_of("\r\n").repeated())
        .map(|(lm, s)| lm.to_string() + &s.iter().collect::<String>());

    let block_comment = just("/*")
        .then(none_of("*/").repeated())
        .then(just("*/"))
        .map(|((st, s), end)| st.to_string() + &s.iter().collect::<String>() + end);

    let comment = choice((
        line_comment,
        block_comment,
    ));

    comment.map(|s| Token::Comment(s))
}

#[cfg(test)]
mod comment_tests {
    use super::*;

    #[test]
    fn test_line_comment() {
        let result = comments().parse("// hello world");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing line comment");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Comment("// hello world".to_string()), "Token not line comment");
    }

    #[test]
    fn test_block_comment() {
        let result = comments().parse("/* hello world */");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing block comment");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::Comment("/* hello world */".to_string()), "Token not block comment");
    }

}

pub fn whitespace() -> impl Parser<char, Token, Error = Simple<char>> {
    //let whitespace = one_of(" \n\t\r").repeated().map(|s| s.iter().collect::<String>());

    let whitespace = choice((
        just(" "),
        just("\n"),
        just("\t"),
        just("\r"),
    ));

    whitespace.map(|s| Token::WhiteSpace(s.to_string()))
}

#[cfg(test)]
mod whitespace_tests {
    use super::*;


    #[test]
    fn test_whitespace() {
        let result = whitespace().parse(" \n\t\r");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing whitespace");
        }

        let token = result.unwrap();

        assert_eq!(token, Token::WhiteSpace(" \n\t\r".to_string()), "Token not whitespace");
    }
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    
    let token = choice((
        keywords(),
        identifiers(),
        whitespace(),
        symbols(),
        operators(),
        literals(),
        comments(),
    ));

    
    token.repeated().then_ignore(end())
    
}


#[cfg(test)]
mod lexer_tests {
    use super::*;


    #[test]
    fn test_assignment() {
        let result = lexer().parse("a = 1");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing assignment");
        }

        let tokens = result.unwrap();

        assert_eq!(tokens, vec![Token::Identifier("a".to_string()), Token::WhiteSpace(" ".to_string()), Token::Assignment, Token::WhiteSpace(" ".to_string()), Token::Number("1".to_string())], "Token not assignment");
    }

    #[test]
    fn test_type_class() {
        let result = lexer().parse("class Monad m { fn (>>=)(m a, fn (a) -> m b) -> m b }");

        if result.is_err() {
            eprintln!("{:?}", result);
            assert!(false, "Error parsing type class");
        }

        let tokens = result.unwrap();

        assert_eq!(tokens, vec![Token::Class, Token::WhiteSpace(" ".to_string()), Token::Identifier("Monad".to_string()), Token::WhiteSpace(" ".to_string()), Token::Identifier("m".to_string()), Token::WhiteSpace(" ".to_string()), Token::CurlyLeft, Token::WhiteSpace(" ".to_string()), Token::Function, Token::WhiteSpace(" ".to_string()), Token::ParenLeft, Token::Identifier(">>=".to_string()), Token::ParenRight, Token::ParenLeft, Token::Identifier("m".to_string()), Token::WhiteSpace(" ".to_string()), Token::Identifier("a".to_string()), Token::Comma, Token::WhiteSpace(" ".to_string()), Token::Function, Token::WhiteSpace(" ".to_string()), Token::ParenLeft, Token::Identifier("a".to_string()), Token::ParenRight, Token::WhiteSpace(" ".to_string()), Token::FunctionReturn, Token::WhiteSpace(" ".to_string()), Token::Identifier("m".to_string()), Token::WhiteSpace(" ".to_string()), Token::Identifier("b".to_string()), Token::ParenRight, Token::WhiteSpace(" ".to_string()), Token::FunctionReturn, Token::WhiteSpace(" ".to_string()), Token::Identifier("m".to_string()), Token::WhiteSpace(" ".to_string()), Token::Identifier("b".to_string()), Token::ParenRight, Token::WhiteSpace(" ".to_string()), Token::CurlyRight], "Token not type class");
    }
    

    

}





