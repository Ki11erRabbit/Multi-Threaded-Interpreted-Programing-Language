use chumsky::prelude::*;

use crate::parser::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    OperatorOrder(usize),// for setting infix function precedence
    Atomic,              // for setting atomicity of variables
    ThreadLocal,         // for setting thread local variables. Not shared(Default)
    ThreadShared,        // for setting thread shared variables. Shared
    ThreadMutable,       // for setting thread mutable variables. Protected with Mutex
    ThreadSpawn,         // for setting if a function will be spawned in a new thread
    Minimal,             // for marking a type class function to be the only necessary function
    Default,             // for marking a type class function to use the default implementation
    Control,             // For defining an effect function that can alter control flow (i.e. exceptions)
    Final,               // For defining what function to call when an effect escapes into main
}



pub fn attribute_parser() -> impl Parser<Token, Vec<Attribute>, Error = Simple<Token>> {

    let basic_attribute = filter_map(|span, token| match token {
        Token::Identifier(name) => Ok(name),
        _ => Err(Simple::custom(span, "Expected identifier")),
    }
    )
        .then(filter_map(|span, token| match token {
            Token::Number(num) => Ok(num),
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier or number")),
        }).repeated())
        .map(|(name, values)| match name.as_str() {
            "Op-Ord" => Attribute::OperatorOrder(values[0].parse::<usize>().unwrap()),
            "Atomic" => Attribute::Atomic,
            "ThreadLocal" => Attribute::ThreadLocal,
            "ThreadShared" => Attribute::ThreadShared,
            "ThreadMutable" => Attribute::ThreadMutable,
            "ThreadSpawn" => Attribute::ThreadSpawn,
            "Minimal" => Attribute::Minimal,
            "Default" => Attribute::Default,
            "Control" => Attribute::Control,
            "Final" => Attribute::Final,
            _ => panic!("Unknown attribute"),
        });
    
            
    let attribute = just(Token::Attribute).ignore_then(basic_attribute);

    attribute.repeated()
}

#[cfg(test)]
mod attribute_parser_tests {
    use super::*;
    use crate::parser::lexer::lexer;

    #[test]
    fn test_single_attribute() {
        let input = "@Atomic";

        let tokens = lexer(input);

        if tokens.is_err() {
            panic!("Lexer error: {:?}", tokens.err());
        }

        let tokens = tokens.unwrap();

        let result = attribute_parser().parse(tokens);

        if result.is_err() {
            panic!("Parser error: {:?}", result.err());
        }

        let result = result.unwrap();

        assert_eq!(result, vec![Attribute::Atomic], "Incorrect attribute");
        
    }

    #[test]
    fn test_single_attribute_with_value() {
        let input = "@Op-Ord 1";

        let tokens = lexer(input);

        if tokens.is_err() {
            panic!("Lexer error: {:?}", tokens.err());
        }

        let tokens = tokens.unwrap();

        let result = attribute_parser().parse(tokens);

        if result.is_err() {
            panic!("Parser error: {:?}", result.err());
        }

        let result = result.unwrap();

        assert_eq!(result, vec![Attribute::OperatorOrder(1)], "Incorrect attribute");
        
    }

    #[test]
    fn test_multiple_attributes() {
        let input = "@Atomic\n@Op-Ord 1";

        let tokens = lexer(input);

        if tokens.is_err() {
            panic!("Lexer error: {:?}", tokens.err());
        }

        let tokens = tokens.unwrap();

        let result = attribute_parser().parse(tokens);

        if result.is_err() {
            panic!("Parser error: {:?}", result.err());
        }

        let result = result.unwrap();

        assert_eq!(result, vec![Attribute::Atomic, Attribute::OperatorOrder(1)], "Incorrect attribute");
        
    }
}
