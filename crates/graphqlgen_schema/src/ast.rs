use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Document {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Definition {
    Type(TypeDef),
    Scalar(ScalarDef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalarDef {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypeDef {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: TypeRef,

    // For query and mutation fields (optional)
    pub arguments: Option<Vec<InputValue>>,

    // For object fields (optional)
    pub directives: Option<Vec<Directive>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directive {
    pub name: String,
    pub arguments: Option<Vec<InputValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeRef {
    Named(String),
    NonNull(Box<TypeRef>),
    List(Box<TypeRef>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValue {
    pub name: String,
    pub value_type: TypeRef,
}
