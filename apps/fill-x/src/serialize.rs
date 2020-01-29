use std::boxed::Box;
use std::collections::HashMap;

pub struct Field {
    name: String,
    type_name: String,
    r#type: String,
}

pub struct Type {
    name: String,
    alias_of_name: String,
    array_of: Box<Type>,
    optionalOf: Box<Type>,
    base_name: String,
    base: String,
    fields: Vec<Field>,
}

pub struct Symbol {
    name: String,
    precision: usize,
}

pub struct Contract {
    actions: HashMap<String, Type>,
    types: HashMap<String, Type>,
}

pub struct Authorization {
    actor: String,
    permission: String,
}

pub struct Action {
    account: String,
    name: String,
    authorization: Vec<Authorization>,
}

pub struct SerializedAction {
    account: String,
    name: String,
    authorization: Vec<Authorization>,
    data: String,
}
