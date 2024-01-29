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

fn main() {
    println!("Hello, world!");
}
