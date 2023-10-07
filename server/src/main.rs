use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use hyper::server::conn::AddrStream;
use serde_json::json;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::ErrorKind};

const SECRET_KEY: &'static str = "secret_key"; // Use a stronger secret in a real-world scenario

struct RateLimiter {
    visitors: Arc<Mutex<HashMap<SocketAddr, u32>>>,
}

impl RateLimiter {
    fn new() -> Self {
        RateLimiter {
            visitors: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn allow(&self, addr: SocketAddr) -> bool {
        let mut visitors = self.visitors.lock().unwrap();
        let counter = visitors.entry(addr).or_insert(0);
        if *counter >= 5 {  // Allow up to 5 requests
            false
        } else {
            *counter += 1;
            true
        }
    }
}

fn authenticate(token: &str) -> bool {
    let validation = Validation {
        iss: Some("my_issuer".to_string()),
        algorithms: vec![Algorithm::HS256],
        ..Default::default()
    };

    match decode::<serde_json::Value>(&token, &DecodingKey::from_secret(SECRET_KEY.as_ref()), &validation) {
        Ok(_data) => true,
        Err(err) => {
            match *err.kind() {
                ErrorKind::InvalidToken => false,  // token is invalid
                _ => false
            }
        }
    }
}

async fn service_handler(req: Request<Body>, client: &hyper::Client<HttpsConnector<HttpConnector>>) -> Result<Response<Body>, hyper::Error> {
    // Example of request transformation: Adding a custom header
    let req = Request::builder()
        .method(req.method())
        .uri(req.uri())
        .header("X-Custom-Header", "My API Gateway")
        .body(req.into_body())
        .unwrap();

    // Forward the transformed request to the mock service
    let resp = client.request(req).await?;

    // Example of response transformation: Append custom JSON
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let data_result: Result<serde_json::Value, _> = serde_json::from_slice(&body_bytes);

    let mut data = match data_result {
        Ok(d) => d,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from("Failed to parse upstream response"))
                .unwrap())
        }
    };

    data["custom"] = json!("This data is added by the gateway");
    Ok(Response::new(Body::from(data.to_string())))
}

async fn handle_request(req: Request<Body>, remote_addr: SocketAddr, rate_limiter: Arc<RateLimiter>, client: Arc<hyper::Client<HttpsConnector<HttpConnector>>>) -> Result<Response<Body>, hyper::Error> {

    if !rate_limiter.allow(remote_addr) {
        return Ok(Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(Body::from("Too many requests"))
            .unwrap());
    }

    println!("Received request from {}:{}", remote_addr.ip(), remote_addr.port());

    // Authentication
    match req.headers().get("Authorization") {
        Some(value) => {
            let token_str = value.to_str().unwrap_or("");
            if !authenticate(token_str) {
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .unwrap());
            }
        },
        None => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Unauthorized"))
                .unwrap());
        }
    }

    // Send the request to the service handler
    service_handler(req, &client).await
}

#[tokio::main]
async fn main() {
    let rate_limiter = Arc::new(RateLimiter::new());
    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let client = Arc::new(client);

    let make_svc = make_service_fn(move |conn: &AddrStream| {
        let remote_addr = conn.remote_addr();
        let rate_limiter = Arc::clone(&rate_limiter);
        let client = Arc::clone(&client);

        let service = service_fn(move |req| handle_request(req, remote_addr, Arc::clone(&rate_limiter), Arc::clone(&client)));
        async { Ok::<_, hyper::Error>(service) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("API Gateway running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
