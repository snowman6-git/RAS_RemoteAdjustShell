use std::{io::Read, net::TcpStream};

fn main() {
    let address: &str = "localhost:9999";
    println!("{}", address);

    match TcpStream::connect(&address) {
        Ok(stream) => {
            loop {
                
            }
        },
        Err(error) => {}

    }
}