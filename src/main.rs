use std::{io::Read, net::TcpStream};
use std::str::from_utf8;

fn main() {
    let address: &str = "localhost:9999";
    println!("{}", address);
    loop {
        match TcpStream::connect(&address) {
            Ok(mut stream) => {
                let mut data = [0 as u8; 1000]; 
                loop {
                    match stream.read(&mut data){
                        Ok(bsize) => {
                            let received_slice = &data[0..bsize];
                            let text = from_utf8(received_slice).unwrap().trim();
                            println!("{}", text); // 디버깅을 위해 text가 무엇인지 확인해볼 수 있음
                        },
                        Err(error) => {
                            println!("{}", error);
                        }
                    }
                }
            },
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}