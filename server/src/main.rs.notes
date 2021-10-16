use http::request::Request;
use server::Server;

fn main() {
//  let string = String::from("127.0.0.1:8080");
//  let string_slice = &string[10..14]; //[10..] would work. also [..5] would work if we wanted a slice from the begining of the 
//  let string_borrow: &str = &string;
//


//  dbg!(&string);
//  dbg!(string_slice);
//  dbg!(string_borrow);
//  dbg!(string_literal);
//ENUMs
//  let get = Method::GET;
//  let delete = Method::DELETE;
//  let post = Method::POST;
//  let post = Method::PUT;
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
mod server {
pub struct Server {
    addr: String,
}

impl Server{
//can have two types of functionalities that we can associate with struct 
//methods and associated functions
//methods are defined in the context of the struct
//methods always take a special first parameter "self"
//associated funcitons are called with "::"
//    fn new(addr: String) -> Server {// upper case Self is an alias for the name of the struct. 
                                    //so we can use Slef instead of the server
//Create an associated function
     pub fn new(addr: String) -> Self { //"Self" is an alias for struct name "Server"
//        Server {
        Self{
//            addr: addr
            addr
        }
    }
//Create a method:
    pub fn run(self) { //<<< the run function takes the ownership of the whole struct due to "self"
                        //if we didn't want that we could do &self or &mut self

        println!("Listening on {}",self.addr);
    }
}
}
mod http {
    pub mod request {
        use super::method::Method;
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
}
}
/*
 * GET /user?id=10 HTTP/1.1\r\n
 HEADERS \r\n
 BODY
 */
pub mod method {
    pub enum Method {
        GET,
        DELETE,
        POST,
        PUT,
        HEAD,
        CONNECT,
        OPTIONS,
        TRACE,
        PATCH,
        }
    }
}
