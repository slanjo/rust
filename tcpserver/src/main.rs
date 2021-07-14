use std::net::TcpListener;
use std::net::TcpStream;// OR we can do - use std::net::{TcpListener, TcpStream}


fn main() {
    let server = TcpListener::bind("127.0.0.1:6000").unwrap();
    println!("Listening on port 6000");
    let connection = server.accept();
    println!("Client connected. Bye!");
}
