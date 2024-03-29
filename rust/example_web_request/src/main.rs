// TODO: I already forgot why this is here
#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

use bytes::Buf as _;
use hyper::Client;
// if you are using this remember to add the configs in .cargo/config
use hyper_tls::HttpsConnector;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://jsonplaceholder.typicode.com/users".parse().unwrap();
    let users = fetch_json(url).await?;
    // print users
    println!("users: {:#?}", users);

    // print the sum of ids
    let sum = users.iter().fold(0, |acc, user| acc + user.id);
    println!("sum of ids: {}", sum);
    Ok(())
}

async fn fetch_json(url: hyper::Uri) -> Result<Vec<User>> {
    let https = HttpsConnector::new();

    //let client = Client::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Fetch the url...
    let res = client.get(url).await?;

    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(res).await?;

    // try to parse as json with serde_json
    let users = serde_json::from_reader(body.reader())?;

    Ok(users)
}

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
}
