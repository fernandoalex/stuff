use mongodb::{
    bson::doc,
    options::{
        ClientOptions, 
        FindOptions
    },
    Client 
};

use serde::{
    Deserialize, 
    Serialize
};

use futures::stream::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {

    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb://root:example@127.0.0.1:27017/admin?w=majority")
            .await?;

    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    println!("Adds something to a new db");

    let db = client.database("books");

    // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Book>("books");

    let books = vec![
        Book {
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
        },
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
        },
    ];

    // Insert the books into "mydb.books" collection, no manual conversion to BSON necessary.
    typed_collection.insert_many(books, None).await?;

    // Query the books in the collection with a filter and an option.
    let filter = doc! { "author": "George Orwell" };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = typed_collection.find(filter, find_options).await?;

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await? {
        println!("title: {}", book.title);
    }
    Ok(())
}
