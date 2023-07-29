use crate::interpreter::{Interpreter};
use crate::parser::lexer::{lexer, Token};

use std::fs::File;
use std::io::Read;
use chumsky::prelude::*;

use super::algabraic_type_parser::{TypeAlias, ProductType, SumType, type_alias_parser, product_type_parser, sum_type_parser};
use super::type_class_parser::{type_class_definition_parser};
use crate::parser::type_class_parser::TypeClass;

enum TopLevelStatement {
    TypeClass(TypeClass),
    TypeAlias(TypeAlias),
    SumType(SumType),
    ProductType(ProductType),
}

fn module_parser() -> impl Parser<Token, Vec<TopLevelStatement>, Error = Simple<Token>> {
    
    choice((
        type_alias_parser().map(TopLevelStatement::TypeAlias),
        sum_type_parser().map(TopLevelStatement::SumType),
        product_type_parser().map(TopLevelStatement::ProductType),
        type_class_definition_parser().map(TopLevelStatement::TypeClass),
    )).repeated()
}




pub fn file_parser(file: &str, interpreter: &mut Interpreter) {
    let mut file = File::open(file).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");

    file_parser_helper(&contents, interpreter);
}

pub fn file_parser_helper(file_contents: &str, interpreter: &mut Interpreter) {
    let tokens = lexer(file_contents).expect("Something went wrong lexing the file");
    
    let module = module_parser().parse(tokens).expect("Something went wrong parsing the file");

    for statement in module {
        match statement {
            TopLevelStatement::TypeClass(type_class) => {
                interpreter.add_typeclass(type_class.name, type_class.functions);
            },
            TopLevelStatement::TypeAlias(type_alias) => {
                interpreter.add_type(type_alias);
            },
            TopLevelStatement::SumType(sum_type) => {
                interpreter.add_type(sum_type.name);
                //TODO: Add constructors for sum types
            },
            TopLevelStatement::ProductType(product_type) => {
                interpreter.add_type(product_type.name);
                //TODO: Add constructors for product types
            },
        }
    }
}


#[cfg(test)]
mod whole_file_parser {
    use super::*;

    #[test]
    fn test_sum_type() {
        let mut interpreter = Interpreter::new();
        let file_contents = "sum type (Maybe a) { Just(a), Nothing }";
        file_parser_helper(file_contents, &mut interpreter);
        let types = interpreter.get_valid_types();
        assert_eq!(types.read().unwrap().len(), 1);

    }

    #[test]
    fn test_product_type() {
        let mut interpreter = Interpreter::new();
        let file_contents = "product type Fixed { right: Int, left: UInt }";
        file_parser_helper(file_contents, &mut interpreter);
        let types = interpreter.get_valid_types();
        assert_eq!(types.read().unwrap().len(), 1);

    }

    #[test]
    fn test_type_alias() {
        let mut interpreter = Interpreter::new();
        let file_contents = "type String = (List Char)";
        file_parser_helper(file_contents, &mut interpreter);
        let types = interpreter.get_valid_types();
        assert_eq!(types.read().unwrap().len(), 1);

    }

    #[test]
    fn test_type_class() {
        let mut interpreter = Interpreter::new();
        let file_contents = "class (Eq a) { fn (==)(a, a) -> Bool\n fn(!=)(a, a) -> Bool }";
        file_parser_helper(file_contents, &mut interpreter);
        let valid_typeclasses = interpreter.get_type_classes();
        assert_eq!(valid_typeclasses.read().unwrap().len(), 1);
    }

    #[test]
    fn test_multiple_statements() {
        let mut interpreter = Interpreter::new();
        let file_contents = "class (Eq a) { fn (==)(a, a) -> Bool\n fn(!=)(a, a) -> Bool }\nsum type (Maybe a) { Just(a), Nothing }\nproduct type Fixed { right: Int, left: UInt }\ntype String = (List Char)";
        let module = file_parser_helper(file_contents, &mut interpreter);
        let valid_typeclasses = interpreter.get_type_classes();
        assert_eq!(valid_typeclasses.read().unwrap().len(), 1);
        let types = interpreter.get_valid_types();
        assert_eq!(types.read().unwrap().len(), 3);
    }
}
