use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}




impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept(){
                Ok((stream, _)) => {
                    let a = [1, 2, 3, 4]

//                    stream.read();
                
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }

}