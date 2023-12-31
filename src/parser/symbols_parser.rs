use chumsky::prelude::*;





#[derive(Debug, PartialEq, Clone,Hash,Eq)]
pub enum Token {
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

pub fn keyword_class() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("class").map(|_| "class".to_string())
}
pub fn keyword_instance() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("instance").map(|_| "instance".to_string())
}
pub fn keyword_default() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("default").map(|_| "default".to_string())
}
pub fn keyword_sum() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("sum").map(|_| "sum".to_string())
}
pub fn keyword_product() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("product").map(|_| "product".to_string())
}
pub fn keyword_type() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("type").map(|_| "type".to_string())
}
pub fn keyword_function() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("fn").map(|_| "fn".to_string())
}
pub fn keyword_match() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("match").map(|_| "match".to_string())
}
pub fn keyword_while() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("while").map(|_| "while".to_string())
}
pub fn keyword_elwhile() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("elwhile").map(|_| "elwhile".to_string())
}
pub fn keyword_for() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("for").map(|_| "for".to_string())
}
pub fn keyword_loop() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("loop").map(|_| "loop".to_string())
}
pub fn keyword_if() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("if").map(|_| "if".to_string())
}
pub fn keyword_elif() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("elif").map(|_| "elif".to_string())
}
pub fn keyword_else() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("else").map(|_| "else".to_string())
}
pub fn keyword_continue() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("continue").map(|_| "continue".to_string())
}
pub fn keyword_break() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("break").map(|_| "break".to_string())
}
pub fn keyword_in() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("in").map(|_| "in".to_string())
}
pub fn keyword_typeis() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("typeis").map(|_| "typeis".to_string())
}
pub fn keyword_effect() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("effect").map(|_| "effect".to_string())
}
pub fn keyword_with() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("with").map(|_| "with".to_string())
}
pub fn keyword_return() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("return").map(|_| "return".to_string())
}
pub fn keyword_mod() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("mod").map(|_| "mod".to_string())
}
pub fn keyword_import() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("import").map(|_| "import".to_string())
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

pub fn operator_assignment() -> impl Parser<char, String, Error = Simple<char>> {
    just("=").map(|_| "=".to_string())
}
pub fn operator_mutable_assignment() -> impl Parser<char, String, Error = Simple<char>> {
    just(":=").map(|_| ":=".to_string())
}
pub fn operator_colon() -> impl Parser<char, String, Error = Simple<char>> {
    just(":").map(|_| ":".to_string())
}
pub fn operator_period() -> impl Parser<char, String, Error = Simple<char>> {
    just(".").map(|_| ".".to_string())
}
pub fn operator_attribute() -> impl Parser<char, String, Error = Simple<char>> {
    just("@").map(|_| "@".to_string())
}
pub fn operator_wildcard() -> impl Parser<char, String, Error = Simple<char>> {
    just("_").map(|_| "_".to_string())
}



#[cfg(test)]
mod operator_tests {
    use super::*;

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


pub fn symbol_bracket_left() -> impl Parser<char, String, Error = Simple<char>> {
    just("[").map(|_| "[".to_string())
}
pub fn symbol_bracket_right() -> impl Parser<char, String, Error = Simple<char>> {
    just("]").map(|_| "]".to_string())
}
pub fn symbol_paren_left() -> impl Parser<char, String, Error = Simple<char>> {
    just("(").map(|_| "(".to_string())
}
pub fn symbol_paren_right() -> impl Parser<char, String, Error = Simple<char>> {
    just(")").map(|_| ")".to_string())
}
pub fn symbol_curly_left() -> impl Parser<char, String, Error = Simple<char>> {
    just("{").map(|_| "{".to_string())
}
pub fn symbol_curly_right() -> impl Parser<char, String, Error = Simple<char>> {
    just("}").map(|_| "}".to_string())
}
pub fn symbol_comma() -> impl Parser<char, String, Error = Simple<char>> {
    just(",").map(|_| ",".to_string())
}
pub fn symbol_semicolon() -> impl Parser<char, String, Error = Simple<char>> {
    just(";").map(|_| ";".to_string())
}
pub fn symbol_function_arrow() -> impl Parser<char, String, Error = Simple<char>> {
    just("->").map(|_| "->".to_string())
}
pub fn symbol_match_arm() -> impl Parser<char, String, Error = Simple<char>> {
    just("=>").map(|_| "=>".to_string())
}
pub fn symbol_namespace() -> impl Parser<char, String, Error = Simple<char>> {
    just("::").map(|_| "::".to_string())
}



fn numbers() -> impl Parser<char, String, Error = Simple<char>> {

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
        .map(|((s, i), suf)| s.to_owned() + &i + &suf);

    let hex_int = sign
        .then(hex_prefix.map(|p| p))
        .then(hex.map(|i| i))
        .then(int_suffix.map(|s| s))
        .map(|(((s, p), i), suf)| s.to_owned() + &p + &i + &suf);

    let oct_int = sign
        .then(oct_prefix.map(|p| p))
        .then(oct.map(|i| i))
        .then(int_suffix.map(|s| s))
        .map(|(((s, p), i), suf)| s.to_owned() + &p + &i + &suf);

    let bin_int = sign
        .then(bin_prefix.map(|p| p))
        .then(bin.map(|i| i))
        .then(int_suffix.map(|s| s))
        .map(|(((s, p), i), suf)| s.to_owned() + &p + &i + &suf);

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
        .map(|((((s, i), p), i2), e)| s.to_owned() + &i + &p + &i2 + &e);


    let float_wo_exp = sign
        .then(dec.map(|i| i))
        .then(dec_point.map(|p| p.to_string()))
        .then(dec.map(|i| i))
        .map(|(((s, i), p), i2)| s.to_owned() + &i + &p + &i2);

    let float = choice((float_exp, float_wo_exp));
        

    let number = choice((float, integer));

        
    number
}



fn strings() -> impl Parser<char, String, Error = Simple<char>> {

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
        .map(|s| s.iter().collect());
    

    string
}



fn chars() -> impl Parser<char, String, Error = Simple<char>> {
    
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
        .map(|s| s.to_string());
    

    char_
}

pub fn literals() -> impl Parser<char, String, Error = Simple<char>> {
    strings()
        .or(chars())
        .or(numbers())
}



pub fn identifiers() -> impl Parser<char, String, Error = Simple<char>> {
    let cant_start_with = none_of::<char, &str,Simple<char>>("0123456789 \n\t\r'\"\\,()[]{}@;:");
    let cant_contain = none_of(" \n\t\r'\"\\,()[]{}@;:");


    //these handle weird edge cases in the parser
    let special_identifiers = choice((
        just("get[]").map(|s| s.to_string()),
        just("set[]").map(|s| s.to_string()),
        just("con[]").map(|s| s.to_string()),
        just("con{}").map(|s| s.to_string()),
        just("add:").map(|s| s.to_string()),
        just("remove:").map(|s| s.to_string()),
        //just("..").map(|s| s.to_string()),
        //just("...").map(|s| s.to_string()),
        //just("..=").map(|s| s.to_string()),
       // just("==").map(|s| s.to_string()),
       // just("!=").map(|s| s.to_string()),
        //just("<=").map(|s| s.to_string()),
        //just(">=").map(|s| s.to_string()),
        //just(">>=").map(|s| s.to_string()),
    ));


    let normal = cant_start_with
        .then(cant_contain.repeated())
        .map(|(c, s)| format!("{}{}", c, s.iter().collect::<String>()));

    /*let basic_identifier = 
        cant_be.not()
            .then(normal)
            .map(|(c,s)| {c.to_string() + &s});*/

    let valid_identifier = choice((
        special_identifiers,
        normal,
    ))
        .map(|s| s);



    let identifier = valid_identifier.validate(|s, span, emit| {
        match s.as_str() {
            ":=" | "::" | "->" | "." | "=" => {
                emit(Simple::custom(span, "identifier can't be built in operator".to_string()));
                s
            },
            "class"|"instance"|"default"|"sum"|"product"|"type"|"fn"|"match"| "while"|"elwhile"|"for"|"loop"|"if"|"elif"|"else"|"continue"|"break"|"in"|"typeis"|"effect"|"with"|"return"|"mod"|"import" => {
                emit(Simple::custom(span, "identifier can't be keyword".to_string()));
                s
            },
            _ => s,
        }
    });
    
        
    

    identifier
}
