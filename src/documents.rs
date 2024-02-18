use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Value {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Text(String),
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Integer(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::Text(s.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Document<Value> {
    pub document_name: String,
    pub fields: Vec<(String, Value)>,
}

impl<Value> Document<Value>
where
    Value: fmt::Debug,
{
    pub fn new(name: &str) -> Self {
        Document {
            document_name: name.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, key: &str, value: T)
    where
        T: Into<Value>,
    {
        self.fields.push((key.to_string(), value.into()));
    }
}

impl<Value> fmt::Debug for Document<Value>
where
    Value: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Document")
            .field("document_name", &self.document_name)
            .field("fields", &self.fields)
            .finish()
    }
}