use mongodb::{
    bson::doc, 
    bson::Document, 
    options::ClientOptions, Client
};

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

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    println!("Print all the db_names");
    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    println!("Adds something to a new db");

    let db = client.database("books");
    let collections = db.collection::<Document>("books");

    let docs = vec![
        doc! {"title": "123", "author":"potato"},
    ];

    collections.insert_many(docs, None).await?;

    Ok(())
}
