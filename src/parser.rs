

use nom::{IResult, bytes::complete::*, combinator::*, sequence::*, multi::*, branch::*, character::complete::*};


#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    String(String),
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

fn operator_parser(input: &str) -> IResult<&str, &str> {
    alt((
        tag("."),
        tag("="),
        tag(":="),
        tag("@"),
        tag(","),
        tag(":"),
        tag("_"),
    ))(input)
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
        tag("&"),
    ))(input)
}

fn number_parser(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| {c.is_numeric() || c == '.' || c == 'x' || c == 'o' || c == 'i' || c == 'u' || c == 'X' || c == 'O' || c == 'I' || c == 'U' ||
               c == 'a' || c == 'b' || c == 'c' || c == 'd' ||
               c == 'e' || c == 'f' || c == 'A' || c == 'B' ||
                          c == 'C' || c == 'D' || c == 'E' || c == 'F'})(input)
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

fn literal_parser(input: &str) -> IResult<&str, &str> {
    alt((
        number_parser,
        string_parser,
        char_parser,
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
    if first == Some('.') && input.chars().nth(1) == Some('.') {
        // This is so that we can parse things like "...", "..", and "..=" but not "."
        return take_till(|c: char| {c.is_whitespace() || c == ',' || c == '"' || c == '\'' ||
                             c == '[' || c == ']' || c == '(' || c == ')' || c == ':' ||
                             c == '@' || c == '{' || c == '}'
        })(input);
    }
    
    alt((
        take_till(|c: char| {c.is_whitespace() || c == '.' || c == ',' || c == '"' || c == '\'' ||
                             c == '[' || c == ']' || c == '(' || c == ')' || c == ';' || c == ':' ||
                             c == '=' || c == '@' || c == '{' || c == '}' || c == '&' 
        }),

    ))(input)
}
