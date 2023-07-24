

use nom::{IResult, bytes::complete::*, combinator::*, sequence::*, multi::*, branch::*, character::complete::*};



//TODO: Change String to &str
#[derive(Debug, PartialEq)]
pub enum Token {
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





fn keyword_parser(input: &str) -> IResult<&str, &str> {
   let mut keyword = alt((
        tag("class"),
        tag("instance"),
        tag("default"),
        tag("sum"),
        tag("product"),
        tag("type"),
        tag("fn"),
        tag("match"),
        tag("while"),
        tag("elwhile"),
        tag("for"),
        tag("loop"),
        tag("if"),
        tag("elif"),
        tag("else"),
        tag("continue"),
        tag("break"),
        tag("in"),
        tag("typeis"),
        tag("effect"),
        tag("with"),
   ));
    let mut keyword = alt((
        keyword,
        tag("return"),
        tag("mod"),
        tag("import")));

    keyword(input)
}

fn is_keyword(input: &str) -> bool {
    match input {
        "class" => true,
        "instance" => true,
        "default" => true,
        "sum" => true,
        "product" => true,
        "type" => true,
        "fn" => true,
        "match" => true,
        "while" => true,
        "elwhile" => true,
        "for" => true,
        "loop" => true,
        "if" => true,
        "elif" => true,
        "else" => true,
        "continue" => true,
        "break" => true,
        "in" => true,
        "typeis" => true,
        "effect" => true,
        "with" => true,
        "return" => true,
        "mod" => true,
        "import" => true,
        _ => false,
    }
}

fn keyword_to_token(input: &str) -> IResult<&str, Token> {
    let (input, keyword) = keyword_parser(input)?;
    match keyword {
        "class" => Ok((input, Token::Class)),
        "instance" => Ok((input, Token::Instance)),
        "default" => Ok((input, Token::Default)),
        "sum" => Ok((input, Token::Sum)),
        "product" => Ok((input, Token::Product)),
        "type" => Ok((input, Token::Type)),
        "fn" => Ok((input, Token::Function)),
        "match" => Ok((input, Token::Match)),
        "while" => Ok((input, Token::While)),
        "elwhile" => Ok((input, Token::ElWhile)),
        "for" => Ok((input, Token::For)),
        "loop" => Ok((input, Token::Loop)),
        "if" => Ok((input, Token::If)),
        "elif" => Ok((input, Token::Elif)),
        "else" => Ok((input, Token::Else)),
        "continue" => Ok((input, Token::Continue)),
        "break" => Ok((input, Token::Break)),
        "in" => Ok((input, Token::In)),
        "typeis" => Ok((input, Token::Typeis)),
        "effect" => Ok((input, Token::Effect)),
        "with" => Ok((input, Token::With)),
        "return" => Ok((input, Token::Return)),
        "mod" => Ok((input, Token::Mod)),
        "import" => Ok((input, Token::Import)),
        _ => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))),
    }
}

fn operator_parser(input: &str) -> IResult<&str, &str> {
    alt((
        tag("."),
        tag("="),
        tag(":="),
        tag("@"),
        tag(":"),
        tag("_"),
    ))(input)
}

fn operator_to_token(input: &str) -> IResult<&str, Token> {
    let (input, operator) = operator_parser(input)?;
    match operator {
        "." => Ok((input, Token::Period)),
        "=" => Ok((input, Token::Assignment)),
        ":=" => Ok((input, Token::MutableAssignment)),
        "@" => Ok((input, Token::Attribute)),
        ":" => Ok((input, Token::Colon)),
        "_" => Ok((input, Token::WildCard)),
        _ => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))),
    }
}


fn symbol_parser(input: &str) -> IResult<&str, &str> {
    alt((
        tag("["),
        tag("]"),
        tag("("),
        tag(")"),
        tag("{"),
        tag("}"),
        tag(";"),
        tag("->"),
        tag("=>"),
        tag("::"),
        tag(","),
    ))(input)
}

fn symbol_to_token(input: &str) -> IResult<&str, Token> {
    let (input, symbol) = symbol_parser(input)?;
    match symbol {
        "[" => Ok((input, Token::BracketLeft)),
        "]" => Ok((input, Token::BracketRight)),
        "(" => Ok((input, Token::ParenLeft)),
        ")" => Ok((input, Token::ParenRight)),
        "{" => Ok((input, Token::CurlyLeft)),
        "}" => Ok((input, Token::CurlyRight)),
        ";" => Ok((input, Token::Semicolon)),
        "->" => Ok((input, Token::FunctionReturn)),
        "=>" => Ok((input, Token::MatchArm)),
        "::" => Ok((input, Token::Namespace)),
        "," => Ok((input, Token::Comma)),
        _ => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))),
    }
}

fn number_parser(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| {c.is_numeric() || c == '.' || c == 'x' || c == 'o' || c == 'i' || c == 'u' || c == 'X' || c == 'O' || c == 'I' || c == 'U' ||
               c == 'a' || c == 'b' || c == 'c' || c == 'd' ||
               c == 'e' || c == 'f' || c == 'A' || c == 'B' ||
                          c == 'C' || c == 'D' || c == 'E' || c == 'F'})(input)
}

fn number_to_token(input: &str) -> IResult<&str, Token> {
    let (input, number) = number_parser(input)?;
    Ok((input, Token::Number(number.to_string())))
}

fn string_parser(input: &str) -> IResult<&str, &str> {
    let mut start = 0;
    let mut end = 0;
    let mut escaped = false;
    if !(input.chars().nth(0) == Some('"')) {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
    }
    for (i, c) in input.chars().enumerate() {
        if c == '"' && !escaped {
            end = i;
            break;
        }
        if c == '\\' {
            escaped = true;
        } else {
            escaped = false;
        }
    }

    if end == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
    } else {
        Ok((&input[end..], &input[start..end]))
    }
}

fn string_to_token(input: &str) -> IResult<&str, Token> {
    let (input, string) = string_parser(input)?;
    Ok((input, Token::String(string.to_string())))
}

fn char_parser(input: &str) -> IResult<&str, &str> {
    let mut start = 0;
    let mut end = 0;
    let mut escaped = false;
    if !(input.chars().nth(0) == Some('\'')) {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
    }
    for (i, c) in input.chars().enumerate() {
        if c == '\'' && !escaped {
            end = i;
            break;
        }
        if c == '\\' {
            escaped = true;
        } else {
            escaped = false;
        }
    }

    if end == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
    } else {
        Ok((&input[end..], &input[start..end]))
    }
}

fn char_to_token(input: &str) -> IResult<&str, Token> {
    let (input, c) = char_parser(input)?;
    Ok((input, Token::Char(c.to_string())))
}

fn literal_parser(input: &str) -> IResult<&str, Token> {
    alt((
        number_to_token,
        string_to_token,
        char_to_token,
    ))(input)
}
        
fn identifier_parser(input: &str) -> IResult<&str, &str> {
    let first = input.chars().nth(0);

    if first == Some('1') || first == Some('2') || first == Some('3') || first == Some('4') ||
        first == Some('5') || first == Some('6') || first == Some('7') || first == Some('8') ||
        first == Some('9') || first == Some('0') || first == Some(',') ||
        first == Some('"') || first == Some('\'') || first == Some('[') || first == Some(']') ||
        first == Some('(') || first == Some(')') || first == Some('@') {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
        }

    let restrict = |c : char| {c.is_whitespace() || c == ',' || c == '"' || c == '\'' ||
                             c == '[' || c == ']' || c == '(' || c == ')' || c == ':' ||
                             c == '@' || c == '{' || c == '}'};
    
    if first == Some('.') && input.chars().nth(1) != None && !restrict(input.chars().nth(1).unwrap()) {
        // This is so that we can parse things like "...", "..", and "..=" but not "."
        return take_till(restrict)(input);
    }
    
    alt((
        take_till(|c: char| {c.is_whitespace() || c == '.' || c == ',' || c == '"' || c == '\'' ||
                             c == '[' || c == ']' || c == '(' || c == ')' || c == ';' || c == ':' ||
                             c == '=' || c == '@' || c == '{' || c == '}'
        }),
        //Getters and setters for the index operator ([])
        tag("get[]"),
        tag("set[]"),
        //Constructor for the square brackets
        tag("con[]"),
        //Constructor for the curly braces
        tag("con{}"),
        //Cons operator and list deconstructor
        tag("add:"),
        tag("remove:")

    ))(input)
}

fn identifier_to_token(input: &str) -> IResult<&str, Token> {
    let (input, identifier) = identifier_parser(input)?;
    Ok((input, Token::Identifier(identifier.to_string())))
}
