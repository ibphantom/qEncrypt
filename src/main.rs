mod encryption;
mod qrng;
mod integrity;

use tiny_http::{Server, Response};

fn main() {
    // Generate a quantum-random key (simulated)
    let master_key = qrng::generate_qrng_key();

    // Use encryption module with our master key
    let plaintext = b"Sensitive Data";
    let encrypted_data = encryption::custom_encrypt(plaintext, &master_key);

    // Calculate integrity check using HMAC-SHA3-512
    let hmac = integrity::calculate_hmac(&encrypted_data, &master_key);

    // Start the HTTP server to serve encrypted data
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Server running on http://0.0.0.0:8080");

    for request in server.incoming_requests() {
        let response = Response::from_string(format!(
            "Encrypted Data: {:?}\nHMAC: {:?}",
            encrypted_data, hmac
        ));
        request.respond(response).unwrap();
    }
}
