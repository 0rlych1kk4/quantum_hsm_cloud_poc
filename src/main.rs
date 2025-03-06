use warp::Filter;
use pqcrypto_ntru::ntruhps2048509 as ntru;
use serde::{Serialize, Deserialize};
use tokio;

#[derive(Serialize, Deserialize)]
struct KeyPair {
    public: Vec<u8>,
    private: Vec<u8>,
}

async fn generate_keypair() -> impl warp::Reply {
    let (pk, sk) = ntru::keypair();
    let response = KeyPair {
        public: pk.as_bytes().to_vec(),
        private: sk.as_bytes().to_vec(),
    };
    warp::reply::json(&response)
}

#[tokio::main]
async fn main() {
    let keypair_route = warp::path("generate_keypair")
        .and(warp::get())
        .and_then(|| async { Ok::<_, warp::Rejection>(generate_keypair().await) });

    println!("Quantum HSM Cloud Service running at http://127.0.0.1:3030");
    warp::serve(keypair_route).run(([127, 0, 0, 1], 3030)).await;
}

