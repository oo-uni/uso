use tiny_http::{Server, Response, Request};
use std::io::Read;

fn handle_request(mut request: Request) {
    // Read the incoming JSON
    let mut content = String::new();
    if let Err(e) = request.as_reader().read_to_string(&mut content) {
        eprintln!("Failed to read request: {}", e);
    }

    // Echo back the payload
    let response = Response::from_string(format!(r#"{{"echo": "{}"}}"#, content))
        .with_header("Content-Type: application/json".parse().unwrap());

    // Send response
    if let Err(e) = request.respond(response) {
        eprintln!("Failed to respond: {}", e);
    }
}

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    println!("Karma Core running on http://localhost:8000");

    for request in server.incoming_requests() {
        handle_request(request);
    }
}
