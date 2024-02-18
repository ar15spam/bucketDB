use db::database::Database;
use db::documents::{Document, Value};
use db::collections::Collection; // Import Collection

fn main() {
    let mut db = Database::new();
    let mut collection = Collection::<Value>::new("Collection1");
    let mut collection2 = Collection::<Value>::new("Collection2");
    let mut doc = Document::<Value>::new("doc1");
    let mut doc2 = Document::<Value>::new("doc2");
    doc.add("name", "Aaron"); // Convert "Aaron" to String
    doc.add("age", 16); // No change needed for numeric types
    doc2.add("name", "Oreo"); // Convert "Oreo" to String
    doc2.add("age", 18); // No change needed for numeric types
    collection.add("doc1", doc);
    collection2.add("doc2", doc2);
    db.add(collection2);
    db.add(collection);
    dbg!(db); 
}
