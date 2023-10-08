# rust-api-gateway
A sample API Gateway built in Rust (work in progress) for learning purposes. You can follow along by reading the introduction article here: https://medium.com/dev-genius/implementing-a-fully-functional-api-gateway-in-rust-part-1-0eb1d9e8b08e.

This API Gateway is built in Rust and serves as a foundational layer for directing incoming HTTP requests to appropriate services, along with providing several essential features for improving security, observability, and control.

This first version is a rudimentary implementation upon which we will build new features along with tutorials in my Blog at www.luissoares.tech and https://medium.com/@luishrsoares

Table of Contents

Features
Setup and Installation
Usage
Future Enhancements

Features

1. Routing

    Requests can be routed to different mock services based on the endpoint path.
    Endpoints like /service1 and /service2 are directed to their respective mock handlers.

2. Rate Limiting

    Implemented an IP-based rate limiter.
    Restricts the number of requests from a specific IP address.
    Responds with "Too many requests" if a limit is exceeded.

3. Logging

    Logs incoming requests.
    Displays the source IP address of the requester.

4. Request & Response Transformation

    Requests: Adds a custom header (X-Custom-Header) before forwarding.
    Responses: Appends custom JSON data to the responses from the services.

5. Authentication

    Utilizes JWT-based authentication.
    Requests must present a valid JWT token in the Authorization header.
    Validates the token's signature and expected claims.

6. HTTPS Client

    Can forward requests to HTTPS services.

7. Error Handling

    Provides appropriate HTTP status codes and error messages for specific scenarios, e.g., missing routes or authentication failures.

Setup and Installation

To set up the API Gateway:

1. Clone the repository.
2. Install the required dependencies using cargo.
3. Run the gateway using cargo run.

Usage

To test the API Gateway, use any HTTP client like curl or Postman.

For JWT authenticated requests:

    curl -H "Authorization: [Your_JWT_Token]" http://localhost:8080/service1

For non-authenticated requests:

    curl http://localhost:8080/service1

Future Enhancements

Caching: Implement caching mechanisms for frequently accessed routes.

Advanced Rate Limiting: Integrate with tools like Redis for distributed rate limiting.

Service Discovery: Dynamically discover and route to services.

Metrics and Monitoring: Integrate with monitoring tools for system health checks and observability.
