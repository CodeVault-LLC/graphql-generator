use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Document {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Definition {
    Type(TypeDef),
    Scalar(ScalarDef),
    Input(TypeDef),
    Interface(TypeDef),
    Union(UnionDef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionDef {
    pub name: String,
    pub members: Vec<TypeRef>,
    pub directives: Option<Vec<Directive>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalarDef {
    pub name: String,
    pub directives: Option<Vec<Directive>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypeDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub directives: Option<Vec<Directive>>,
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
pub enum Value {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Enum(String),
    Object(Vec<(String, Value)>),
    List(Vec<Value>),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValue {
    pub name: String,
    pub value_type: TypeRef,
    pub default_value: Option<Value>,
}
