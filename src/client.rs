use reqwest::Client;
use serde::Deserialize;
use tokio;

#[derive(Deserialize)]
struct KeyPair {
    public: Vec<u8>,
    private: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res: KeyPair = client.get("http://127.0.0.1:3030/generate_keypair")
        .send().await?
        .json().await?;

    println!("Public Key: {:?}", res.public);
    println!("Private Key: {:?}", res.private);
    Ok(())
}

