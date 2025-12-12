use std::io::{self, Read, Write};
use std::time::{Duration};
use std::{net::TcpStream, io::ErrorKind};
use std::{str::from_utf8};
use std::thread::{sleep};

fn reconnect(){
    let mut reconnect_ment = "try to connect.".to_string();
    for i in 1..=3 { //3초 다 채우고 연결하게 할건지 1초에 한번씩 연결시도 할건지 고민하기 
        print!("{}", reconnect_ment);
        reconnect_ment.push('.');
        io::stdout().flush().unwrap();
        print!("\r"); 
        sleep(Duration::from_secs(1));
    }
    print!("\r\x1b[K");
}

fn main() {
    // let address: &str = "localhost:9999";
    let address = "175.206.180.193:9999".parse().unwrap(); //차후 오류/예외처리하기
    println!("{}", address);
    'master_connect: loop {
        match TcpStream::connect_timeout(&address, Duration::from_secs(1)) {
            Ok(mut stream) => {
                println!("Connected!");
                let mut data = [0 as u8; 1000]; 
                'streaming: loop {
                    match stream.read(&mut data){
                        Ok(bsize) => {
                            if bsize == 0 {
                                println!("connection lost!");
                                break 'streaming;
                            }
                            let received_slice = &data[0..bsize];
                            let text = from_utf8(received_slice).unwrap().trim();
                            println!("{}", text); // 디버깅을 위해 text가 무엇인지 확인해볼 수 있음
                        },
                        Err(error) => match error.kind(){
                            ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset => {
                                println!("{}", error);
                                println!("signal lost from: {}", address);
                                break 'streaming;
                            },
                            _ => {
                                println!("{}", error);
                            }
                            
                            // break 'streaming;

                        }
                    }
                }
            },
            Err(error) => match error.kind(){
                ErrorKind::TimedOut | ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset => { 
                    reconnect();
                },
                _ => {
                    println!("{}", error);
                }
            }
        }
    }
}