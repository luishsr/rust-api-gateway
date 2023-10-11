use std::time::{Duration, SystemTime};
use jsonwebtoken::{encode, Header, EncodingKey};
use hyper::{Client, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

const SECRET_KEY: &'static str = "secret_key";  // Must match the secret in the API Gateway

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    exp: usize,
}

#[tokio::main]
async fn main() {
    let claims = Claims {
        sub: "1234567890".to_string(),
        iss: "my_issuer".to_string(),
        exp: (SystemTime::now() + Duration::from_secs(3600))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize, // Expires in 1 hour
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY.as_ref())).unwrap();
    println!("Token: {}", token);

    let client = {
        let https = HttpsConnector::new();
        Client::builder().build::<_, hyper::Body>(https)
    };

    let request = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:8080/hello_service")
        .header("Authorization", token)
        .body(hyper::Body::empty())
        .expect("Request builder failed.");

    let response = client.request(request).await.expect("Request failed.");
    println!("Response: {:?}", response.status());

    let bytes = hyper::body::to_bytes(response.into_body()).await.expect("Failed to read response.");
    let string = String::from_utf8_lossy(&bytes);
    println!("Response Body: {}", string);
}
