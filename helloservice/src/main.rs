use hyper::{Request, Response, Server};
use hyper::body::Body;
use hyper::service::{make_service_fn, service_fn};
use hyper::client::Client;

static API_GATEWAY_ADDRESS: &str = "http://localhost:8080/register_service";  // Adjust as per your setup
static SERVICE_NAME: &str = "hello_service";
static SERVICE_ADDRESS: &str = "http://localhost:9090";  // The address where this service runs

async fn handle_hello(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from(r#"{"message": "Hello from the Service!"}"#)))
}

#[tokio::main]
async fn main() {
    // Register with the API Gateway's service registry
    let client = Client::new();
    let req_body = format!("{},{}", SERVICE_NAME, SERVICE_ADDRESS);

    println!("Hello Service registering...");

    let request = Request::builder()
        .method("POST")
        .uri(API_GATEWAY_ADDRESS)
        .body(Body::from(req_body))
        .unwrap();

    if let Err(e) = client.request(request).await {
        eprintln!("Failed to register the service: {}", e);
    } else {
        println!("Hello Service registered!");
    }

    // Start the Hello Service
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handle_hello)) }
    });

    let addr = ([127, 0, 0, 1], 9090).into();  // This service will run on port 9090
    let server = Server::bind(&addr).serve(make_svc);

    println!("Hello Service running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
