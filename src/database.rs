use crate::collections::Collection;
use crate::documents::Value;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Database {
    collections: Vec<Collection<Value>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            collections: Vec::new(),
        }
    }

    pub fn add(&mut self, collection: Collection<Value>) {
        self.collections.push(collection);
    }
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Database")
            .field("collections", &self.collections)
            .finish()
    }
}
