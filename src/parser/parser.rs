use chumsky::prelude::*;
use crate::parser::lexer::{Token, keywords, comments, whitespace, identifiers, literals, operators, symbols};

pub enum Type {
    WithTypeClass {
        class: String,
        parameters: Vec<Type>,
    },
    Function {
        parameters: Vec<(Option<String>, Type)>,
        effects: Vec<String>,
        return_type: Box<Type>,
    },
    WithoutTypeClass {
        name: String,
    },
    Tuple {
        types: Vec<Type>,
    },
    TypeWithGeneric {
        name: String,
        parameters: Vec<Type>,
    },
    
}

pub enum TopLevelStatement {
    ImmutableGlobal(Assignment),
    MutableGlobal(Assignment),
    FunctionDefinition(FunctionDefinition),
    TypeClassDefinition(TypeClassDefinition),
    TypeClassInstance(TypeClassInstance),
    TypeDefinition(TypeDefinition),
    Import(Import),
    EffectDefinition(EffectDefinition),
    ModuleDeclaration(ModuleDeclaration),
    Comment(String),
}

pub struct ModuleDeclaration {
    pub name: String,
    pub body: Vec<TopLevelStatements>,
}

pub struct Import {
    pub path: Vec<String>,
}

pub enum TypeDefinition {
    Alias(Type, Type),
    Product(Vec<(String, Type)>),
    Sum(Vec<(String, Type)>),
}

pub struct TypeClassDefinition {
    pub name: String,
    pub parameters: Vec<Type>,
    pub functions: Vec<FunctionDefinition>,
}

pub struct TypeClassInstance {
    pub name: String,
    pub parameters: Vec<Type>,
    pub functions: Vec<FunctionDefinition>,
}

pub struct EffectDefinition {
    pub name: String,
    pub parameters: Vec<Type>,
    pub functions: Vec<FunctionDefinition>,
}

pub struct Assignment {
    pub name: String,
    pub value: String,//TODO: change to expression
}



pub enum FunctionDefinition {
    //Primarilly for type classes
    Prototype {
        attributes: Vec<String>,
        infix: bool,
        name: String,
        parameters: Vec<(Type)>,
        effects: Vec<String>,//change to effect
        return_type: Type,
    },
    //For everything else
    Definition {
        attributes: Vec<String>,
        infix: bool,
        name: String,
        parameters: Vec<(String, Type)>,
        effects: Vec<String>,//change to effect
        return_type: Type,
        body: CodeBlock,
    },
}

//TODO: implement
pub struct CodeBlock {
}



/*pub fn parse_module() -> impl Parser<char, TopLevelStatement, Error = Simple<char>> {

    let module = keywords().just(Token::Mod)
        .then(identifiers().map(|x| x.to_string()))
        .then(symbols().just(Token::LeftBrace))
        .then(toplevel_statements())
        .then(symbols().just(Token::RightBrace))
        .map(|((_, name), (_, body))| ModuleDeclaration { name, body }).map(|x| TopLevelStatement::ModuleDeclaration(x));

}*/
/// This function parses a single generic type list, e.g. `<a, b, c>`
/// This should also parse this <List <a>, b, c>
fn parse_generic_type_list() -> impl Parser<char, Vec<Type>, Error = Simple<char>> {

    let list = just("<").padded().ignore()
        .then(identifiers().map(|x| x.to_string()).padded())
        .or(parse_type_with_generic())
        .then(symbols().just(Token::Comma).padded())
        .or(just(">")).padded().ignore();
    
    let generic_type_list = just("<").padded()
        .then(list.repeated())
        .then(just(">")).padded()
        .map(|((_, types), _)| types.into_iter().map(|x| Type::WithoutTypeClass { name: x }).collect());

    generic_type_list
}

/// This should parse something like this List <a> or List<Int>
fn parse_type_with_generic() -> impl Parser<char, Type, Error = Simple<char>> {

    let type_with_generic = identifiers().map(|x| x.to_string()).padded()
        .then(parse_generic_type_list()).padded()
        .map(|(name, parameters)| Type::TypeWithGeneric { name, parameters });

    type_with_generic
}

/// This function parses one of the following:
/// a
/// fn(a, b, c) exn -> d
/// (a, b, c)
/// Eq <a>
/// Eq <List <a>>
/// List <a>
fn parse_type() -> Parser<char, Type, Error = Simple<char>> {

    let tuple_body = parse_type().padded()
        .then(symbols().just(Token::Comma).padded())
        .or(symbols().just(Token::ParenRight).padded().ignore());

    let empty_tuple = symbols().just(Token::ParenLeft).padded()
        .then(symbols().just(Token::ParenRight).padded())
        .map(|_| Type::Tuple { types: vec![] });

    let filled_tuple = symbols().just(Token::ParenLeft).padded()
        .then(tuple_body.repeated())
        .then(symbols().just(Token::ParenRight).padded())
        .map(|((_, types), _)| Type::Tuple { types });

    let tuple = choice((empty_tuple, filled_tuple));

    let single_type = identifiers().map(|x| x.to_string()).padded()
        .map(|x| Type::WithoutTypeClass { name: x });

    let type_with_type_class = identifiers().map(|x| x.to_string()).padded()
        .then(parse_generic_type_list()).padded()
        .map(|(name, parameters)| Type::WithTypeClass { name, parameters });

    let function_type = keywords().just(Token::Function).padded()
        .then(tuple)
        .then(parse_effects().padded())
        .then(symbols().just(Token::FunctionReturn).padded())
        .then(parse_type().padded())
        .map(|((((_, parameters), _), effects), return_type)| Type::Function { parameters, effects, return_type });


    let types = choice((type_with_type_class, function_type,  
        
}

fn parse_attributes() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    let attribute = symbols().just(Token::Attribute).then(identifiers().padded().repeated());

    attribute
}

fn parse_infix_function_name() -> impl Parser<char, String, Error = Simple<char>> {

    let infix = symbols().just(Token::ParenLeft).padded()
        .then(identifiers().map(|x| x.to_string()).padded())
        .then(symbols().just(Token::ParenRight).padded());

    infix.map(|((_, name), _)| name)
}

fn parse_function_name() -> impl Parser<char, String, Error = Simple<char>> {

    let function_name = identifiers().map(|x| x.to_string()).padded();

    function_name
}

fn parse_function_arguments() -> impl Parser<char, Vec<(Option<String>, Option<Type>)>, Error = Simple<char>> {

    let argument_wo_type = identifiers().map(|x| x.to_string()).padded()
        .map(|x| (Some(x), None));

    let argument_with_type = identifiers().map(|x| x.to_string()).padded()
        .then(parse_type_statement().padded())
        .map(|((name, _), typ)| (Some(name), Some(typ)));
    

    let argument_just_type = parse_type().padded()
        .map(|x| (None, Some(x)));

    let argument = argument_wo_type.or(argument_with_type).or(argument_just_type)
        .then(symbols().just(Token::Comma).padded())
        .or(symbols().just(Token::RightParen).padded().ignore());

    let argument_list = symbols().just(Token::LeftParen).padded()
        .then(argument.repeated())
        .then(symbols().just(Token::RightParen).padded());
    
}


fn parse_type_class_function_dec() -> impl Parser<char, FunctionDefinition, Error = Simple<char>> {

    let infix_function = parse_infix_function_name().padded()
        .then(parse_function_arguments());

    let function = parse_function_name().padded()
        .then(parse_function_arguments());


}
    

fn parse_type_class() -> impl Parser<char, TopLevelStatement, Error = Simple<char>> {

    let class = keywords().just(Token::Class)
        .then(whitespace().repeated())
        .then(identifiers().map(|x| x.to_string()))
        .then(parse_generic_type_list())
        .then(symbols().just(Token::LeftBrace))
        .
        

}
