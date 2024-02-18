use std::fmt;
use serde::{Deserialize, Serialize};
use crate::documents::Document;
use std::marker::PhantomData; // Import PhantomData

#[derive(Serialize, Deserialize)]
pub struct Collection<V> {
    pub collection_name: String,
    pub documents: Vec<(String, Document<V>)>,
    _phantom: PhantomData<V>, // Include PhantomData<V> to use the type parameter
}

impl<V> Collection<V> {
    pub fn new(name: &str) -> Self {
        Collection {
            collection_name: name.to_string(),
            documents: Vec::new(),
            _phantom: PhantomData, // Initialize PhantomData
        }
    }

    pub fn add(&mut self, document_name: &str, document: Document<V>) {
        self.documents.push((document_name.to_string(), document));
    }
}

impl<V> fmt::Debug for Collection<V>
where
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Collection")
            .field("collection_name", &self.collection_name)
            .field("documents", &self.documents)
            .finish()
    }
}
