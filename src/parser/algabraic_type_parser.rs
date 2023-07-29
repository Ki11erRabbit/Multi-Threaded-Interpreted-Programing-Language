use chumsky::prelude::*;

use crate::parser::lexer::Token;
use crate::parser::type_parser::{type_parser};

use crate::types::{Type, Value};



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumType {
    name: Type,
    variants: Vec<(String, Option<Type>)>,
}




fn sum_type_parser() -> impl Parser<Token, SumType, Error = Simple<Token>> {


    let variant_parser =
        filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        })
        .then(type_parser())
            .map(|(name, typ)| (name, Some(typ)))
            .or(
                filter_map(|span, token| match token {
                    Token::Identifier(name) => Ok(name),
                    _ => Err(Simple::custom(span, "Expected identifier".to_string())),
                })
                .map(|name| (name, None))
            )
            .labelled("variant");


    let sum_type = just(Token::Sum)
        .ignore_then(type_parser())
        .then_ignore(just(Token::CurlyLeft))
        .then(variant_parser
              .separated_by(just(Token::Comma)))
        .then_ignore(just(Token::CurlyRight))
        .map(|(name, variants)| SumType { name, variants })
        .labelled("sum type");

    sum_type
}


#[cfg(test)]
mod sum_type_tests {
    use super::*;
    use crate::parser::lexer::lexer;

    #[test]
    fn sum_type_test() {
        let input = "sum type a { A, B, C }";

        let lexer_result = lexer(input);

        if lexer_result.is_err() {
            assert!(false,"Lexer error: {:?}", lexer_result.err());
        }

        let tokens = lexer_result.unwrap();

        let result = sum_type_parser().parse(tokens);

        if result.is_err() {
            assert!(false,"Parser error: {:?}", result.err());
        }

        let sum_type = result.unwrap();

        assert_eq!(sum_type.name, Type::Single("a".to_string()), "Name is not correct");
        assert_eq!(sum_type.variants.len(), 3, "Number of variants is not correct");
        assert_eq!(sum_type.variants[0].0, "A".to_string(), "Variant is not A");
        assert_eq!(sum_type.variants[1].0, "B".to_string(), "Variant is not B");
        assert_eq!(sum_type.variants[2].0, "C".to_string(), "Variant is not C");
    }

    #[test]
    fn sum_type_with_type_test() {
        let input = "sum type a { A, B, C(Int) }";

        let lexer_result = lexer(input);

        if lexer_result.is_err() {
            assert!(false,"Lexer error: {:?}", lexer_result.err());
        }

        let tokens = lexer_result.unwrap();

        let result = sum_type_parser().parse(tokens.clone());

        if result.is_err() {
            eprintln!("Tokens: {:?}", tokens);
            assert!(false,"Parser error: {:?}", result.err());
        }

        let sum_type = result.unwrap();

        assert_eq!(sum_type.name, Type::Single("a".to_string()), "Name is not correct");
        assert_eq!(sum_type.variants.len(), 3, "Number of variants is not correct");
        assert_eq!(sum_type.variants[0].0, "A".to_string(), "Variant is not A");
        assert_eq!(sum_type.variants[1].0, "B".to_string(), "Variant is not B");
        assert_eq!(sum_type.variants[2].0, "C".to_string(), "Variant is not C");
        assert_eq!(sum_type.variants[2].1, Some(Type::Tuple(vec![Type::Single("Int".to_string())])), "Variant C does not hava a Tuple of Int");
    }

    #[test]
    fn test_maybe_type() {
        let input = "sum type (Maybe a) { Just(a), Nothing }";

        let lexer_result = lexer(input);

        if lexer_result.is_err() {
            assert!(false,"Lexer error: {:?}", lexer_result.err());
        }

        let tokens = lexer_result.unwrap();

        let result = sum_type_parser().parse(tokens.clone());

        if result.is_err() {
            eprintln!("Tokens: {:?}", tokens);
            assert!(false,"Parser error: {:?}", result.err());
        }

        let sum_type = result.unwrap();

        assert_eq!(sum_type.name, Type::TypeList{ name: Box::new(Type::Single("Maybe".to_string())), parameters: vec![Type::Single("a".to_string())] }, "Name is not correct");
        assert_eq!(sum_type.variants.len(), 2, "Number of variants is not correct");
        assert_eq!(sum_type.variants[0].0, "Just".to_string(), "Variant is not Just");
        assert_eq!(sum_type.variants[1].0, "Nothing".to_string(), "Variant is not Nothing");

    }

}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductType {
    name: Type,
    fields: Vec<(String, Type)>,
}


pub fn product_type_parser() -> impl Parser<Token, ProductType, Error = Simple<Token>> {

    let field_parser =
        filter_map(|span, token| match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(Simple::custom(span, "Expected identifier".to_string())),
        })
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|(name, typ)| (name, typ))
        .labelled("field");

    let product_type = just(Token::Product)
        .ignore_then(type_parser())
        .then_ignore(just(Token::CurlyLeft))
        .then(field_parser
              .separated_by(just(Token::Comma)))
        .then_ignore(just(Token::CurlyRight))
        .map(|(name, fields)| ProductType { name, fields })
        .labelled("product type");

    product_type
}


#[cfg(test)]
mod product_type_tests {
    use super::*;
    use crate::parser::lexer::lexer;

    #[test]
    fn product_type_test() {
        let input = "product type a { a: Int, b: Int }";

        let lexer_result = lexer(input);

        if lexer_result.is_err() {
            assert!(false,"Lexer error: {:?}", lexer_result.err());
        }

        let tokens = lexer_result.unwrap();

        let result = product_type_parser().parse(tokens);

        if result.is_err() {
            assert!(false,"Parser error: {:?}", result.err());
        }

        let product_type = result.unwrap();

        assert_eq!(product_type.name, Type::Single("a".to_string()), "Name is not correct");
        assert_eq!(product_type.fields.len(), 2, "Number of fields is not correct");
        assert_eq!(product_type.fields[0].0, "a".to_string(), "Field is not a");
        assert_eq!(product_type.fields[1].0, "b".to_string(), "Field is not b");
    }

    #[test]
    fn product_type_with_type_test() {
        let input = "product type a { a: Int, b: (Maybe Int) }";

        let lexer_result = lexer(input);

        if lexer_result.is_err() {
            assert!(false,"Lexer error: {:?}", lexer_result.err());
        }

        let tokens = lexer_result.unwrap();

        let result = product_type_parser().parse(tokens);

        if result.is_err() {
            assert!(false,"Parser error: {:?}", result.err());
        }

        let product_type = result.unwrap();

        assert_eq!(product_type.name, Type::Single("a".to_string()), "Name is not correct");
        assert_eq!(product_type.fields.len(), 2, "Number of fields is not correct");
        assert_eq!(product_type.fields[0].0, "a".to_string(), "Field is not a");
        assert_eq!(product_type.fields[1].0, "b".to_string(), "Field is not b");
        assert_eq!(product_type.fields[1].1, Type::TypeList{ name: Box::new(Type::Single("Maybe".to_string())), parameters: vec![Type::Single("Int".to_string())] }, "Field b does not hava a Maybe of Int");
    }

}



pub fn type_alias_parser() -> impl Parser<Token, Type, Error = Simple<Token>> {

    let type_alias = just(Token::Type)
        .ignore_then(type_parser())
        .then_ignore(just(Token::Assignment))
        .then(type_parser())
        .map(|(name, typ)| Type::Alias(Box::new(name), Box::new(typ)))
        .labelled("type alias");
    
    type_alias
}


#[cfg(test)]
mod type_alias_test {
    use super::*;
    use crate::parser::lexer::lexer;

    #[test]
    fn type_alias_test() {
        let input = "type a = Int";

        let lexer_result = lexer(input);

        if lexer_result.is_err() {
            assert!(false,"Lexer error: {:?}", lexer_result.err());
        }

        let tokens = lexer_result.unwrap();

        let result = type_alias_parser().parse(tokens);

        if result.is_err() {
            assert!(false,"Parser error: {:?}", result.err());
        }

        let type_alias = result.unwrap();

        assert_eq!(type_alias, Type::Alias(Box::new(Type::Single("a".to_string())), Box::new(Type::Single("Int".to_string()))), "Type alias is not correct");
    }

    #[test]
    fn test_string() {
        let input = "type String = (List Char)";

        let lexer_result = lexer(input);

        if lexer_result.is_err() {
            assert!(false,"Lexer error: {:?}", lexer_result.err());
        }

        let tokens = lexer_result.unwrap();

        let result = type_alias_parser().parse(tokens);

        if result.is_err() {
            assert!(false,"Parser error: {:?}", result.err());
        }

        let type_alias = result.unwrap();

        assert_eq!(type_alias, Type::Alias(Box::new(Type::Single("String".to_string())), Box::new(Type::TypeList{ name: Box::new(Type::Single("List".to_string())), parameters: vec![Type::Single("Char".to_string())] })), "Type alias is not correct");
    }
    
}
