use dashmap::DashMap;

#[derive(Clone)]
pub struct Variable {
    typ: Typ
}

#[derive(Clone)]
pub struct Enum {
    members: DashMap<String, String> // Enum, DefaultValue
}

#[derive(Clone)]
pub struct Type {
    members: DashMap<String, String> // Name, Type
}

#[derive(Clone)]
pub enum Typ {
    Enum(Enum),
    Type(Type)
}
