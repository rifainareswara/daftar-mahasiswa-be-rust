use mongodb::{Client, Database};

pub async fn connect_db(uri: &str) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(uri).await?;
    Ok(client.database("daftar_mahasiswa"))
}
