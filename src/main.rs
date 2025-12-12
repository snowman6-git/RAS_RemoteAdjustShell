use std::{io::Write, net::{TcpListener, TcpStream}, thread, env::args};

fn handle_connection(mut stream: TcpStream){
    let ip = stream.peer_addr();
    //소켓에 보내는거 만으로 연결이 됐음을 확정지을수 있는지 확인 ex. 연결이 끊어져도 보내지는가 (보내는거만으로 연결이 됌을 확정지을 수 있는가)
    let handshake_msg = stream.write("aa2iswork".as_bytes());
    match handshake_msg {
        Ok(_) => {
            println!("{}: HandShake Complete!", ip.unwrap());
            loop {
                let mut msg = String::from("");
                std::io::stdin().read_line(&mut msg).unwrap();
                stream.write(msg.as_bytes()).unwrap();
            }
        },
        Err(er) => {println!("{}: {}", ip.unwrap(), er)}  
    }
    
    loop {
        let mut msg = String::from("");
        std::io::stdin().read_line(&mut msg).unwrap();
        stream.write(msg.as_bytes()).unwrap();
    }
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
    let args: Vec<String> = args().collect();
    println!("RAS is Loading...");

    let master_ip = &args[1];
    let master_port = &args[2];

    let master_addr: String = format!(
        "{master_ip}:{master_port}",
    );
    println!("RAS is try to open in: '{}'", master_addr);
    
    let _listner = listen_up(&master_addr);

    // 들어오는 애들 잡음
    for stream in _listner.incoming() {
        let stream = stream.unwrap();
        // 스레드 생성
        thread::spawn(|| {
            // 들어온 애 주소
            let incoming_ip_read = stream.peer_addr();
            match incoming_ip_read {
                Ok(ip) => {
                    println!("{} is incoming...", ip);
                },
                Err(error) => {
                    println!("{}", error)
                }
            };
            // 절로 던져
            handle_connection(stream);
        });
    }
    println!("Hello, world!");

}
