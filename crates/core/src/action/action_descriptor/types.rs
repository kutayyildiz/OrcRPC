use serde::{Deserialize, Serialize};

use crate::action::ActionKind;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum PrimitiveDescriptor {
    Null,
    Bool,
    String,
    Integer,
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ValueDescriptor {
    Primitive(PrimitiveDescriptor),
    Array(Box<ValueDescriptor>),
    Object(NestedObjectDescriptor),
    Map(Box<ValueDescriptor>),
    OneOf(Vec<ValueDescriptor>),
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FieldDescriptor {
    pub name: String,
    pub ty: ValueDescriptor,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct NestedObjectDescriptor {
    pub fields: Vec<FieldDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ParamsObjectDescriptor {
    pub required_fields: Vec<FieldDescriptor>,
    pub optional_fields: Vec<FieldDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ParamsDescriptor {
    Value(ValueDescriptor),
    Object(ParamsObjectDescriptor),
}

pub type OkDescriptor = ValueDescriptor;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ActionDescriptor {
    pub kind: ActionKind,
    pub params: Option<ParamsDescriptor>,
    pub ok: Option<OkDescriptor>,
}
