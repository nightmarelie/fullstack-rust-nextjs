use postgres::{Client, NoTls};
use postgres::Error;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

//Model: User struct with id, name, email, password
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
    password: Option<String>,
}

// CONSTANTS
const DATABASE_URL: &str = env!("DATABASE_URL");
// Define some responses
const OK_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

fn main() {
    // set database
    if let Err(_) = set_database() {
        println!("Error setting up database");
        return;
    }

    //start server and print port
        let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
        println!("Server listening on port 8080");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream);
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
}
