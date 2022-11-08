use mongodb::sync::{Client, Database};

pub fn get_db_client() -> Database {
    let db_connection_string =
        dotenv::var("MONGO_CONNECTION_STRING").expect("Missing env MONGO_CONNECTION_STRING");

    let client = Client::with_uri_str(db_connection_string).unwrap();

    let db = client.database("highlights");

    db
}
