


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



/// This function parses a single generic type list, e.g. `<a, b, c>`
/// This should also parse this <List <a>, b, c>
fn parse_type_list() -> impl Parser<char, Vec<Type>, Error = Simple<char>> {

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
fn parse_type_with_list() -> impl Parser<char, Type, Error = Simple<char>> {

    let type_with_generic = identifiers().map(|x| x.to_string()).padded()
        .then(parse_generic_type_list()).padded()
        .map(|(name, parameters)| Type::TypeWithGeneric { name, parameters });

    type_with_generic
}
