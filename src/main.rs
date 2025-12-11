use std::{net::{TcpListener, TcpStream}, thread};

fn handle_connection(stream: TcpStream){
}

fn listen_up(master_addr: &str) -> TcpListener{
    match TcpListener::bind(master_addr) {
        Ok(listener) => {
            println!("RAS now open! [ {} ] ", master_addr);
            return listener
        }, 
        Err(e) => {
            eprintln!("RAS Failed to open... [{}]", e);
            std::process::exit(1); 
        }
    };

}

fn main() {
    // 리스닝 아이피:포트
    println!("RAS is Loading...");

    const MASTER_IP: &str = "localhost";
    const MASTER_PORT: &str = "9999";
    let master_addr: String = format!(
        "{MASTER_IP}:{MASTER_PORT}",
    );
    println!("RAS is try to open in: '{}'", master_addr);
    
    let _listner = listen_up(&master_addr);

    // 들어오는 애들 잡음
    for stream in _listner.incoming() {
        let stream = stream.unwrap();
        // 스레드 생성
        thread::spawn(|| {
            // 들어온 애 주소
            println!("{} is connected!", stream.peer_addr().unwrap());
            // 절로 던져
            handle_connection(stream);
        });
    }
    println!("Hello, world!");

}
