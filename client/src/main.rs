use jsonwebtoken::{encode, Header, EncodingKey};
use hyper::{Client, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

const SECRET_KEY: &'static str = "secret_key";  // Must match the secret in the API Gateway

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
}

#[tokio::main]
async fn main() {
    let claims = Claims {
        sub: "1234567890".to_string(),
        iss: "my_issuer".to_string(),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY.as_ref())).expect("Token generation failed");

    let client = {
        let https = HttpsConnector::new();
        Client::builder().build::<_, hyper::Body>(https)
    };

    let request = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:8080/service1")
        .header("Authorization", token)
        .body(hyper::Body::empty())
        .expect("Request builder failed.");

    let response = client.request(request).await.expect("Request failed.");
    println!("Response: {:?}", response.status());

    let bytes = hyper::body::to_bytes(response.into_body()).await.expect("Failed to read response.");
    let string = String::from_utf8_lossy(&bytes);
    println!("Response Body: {}", string);
}
