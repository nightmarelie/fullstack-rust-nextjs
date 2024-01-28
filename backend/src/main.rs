use postgres::{Client, NoTls};
use postgres::Error;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

fn main() {
    println!("Hello, world!");
}
