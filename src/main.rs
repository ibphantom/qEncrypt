mod encryption;
mod qrng;
mod integrity;

use tiny_http::{Server, Response, Method};
use serde_json::json;
use std::io::Read;

fn main() {
    // Start the HTTP server to serve encrypted data
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Server running on http://0.0.0.0:8080");

    for mut request in server.incoming_requests() {
        match request.method() {
            // Serve the HTML form for submitting data
            Method::Get => {
                let html = r#"
                <!DOCTYPE html>
                <html>
                    <head><title>Secure App</title></head>
                    <body>
                        <h1>Submit Data for Encryption</h1>
                        <form action="/" method="post">
                            <label for="data">Enter data to encrypt:</label><br>
                            <input type="text" id="data" name="data"><br><br>
                            <input type="submit" value="Encrypt">
                        </form>
                    </body>
                </html>"#;
                let response = Response::from_string(html).with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
                request.respond(response).unwrap();
            },
            // Handle the data submission and return encrypted data
            Method::Post => {
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).unwrap();
                let data = content.split('=').nth(1).unwrap_or("").as_bytes();

                // Generate a quantum-random key (simulated)
                let master_key = qrng::generate_qrng_key();

                // Encrypt data using custom encryption module
                let encrypted_data = encryption::custom_encrypt(data, &master_key);

                // Calculate integrity check using HMAC-SHA3-512
                let hmac = integrity::calculate_hmac(&encrypted_data, &master_key);

                // Format the JSON response
                let json_response = json!({
                    "input": String::from_utf8_lossy(data),
                    "encrypted_data": encrypted_data,
                    "hmac": hmac,
                    "message": "Your data has been encrypted successfully."
                });

                let response = Response::from_string(json_response.to_string())
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                request.respond(response).unwrap();
            },
            _ => {
                let response = Response::from_string("Unsupported HTTP method").with_status_code(405);
                request.respond(response).unwrap();
            }
        }
    }
}
