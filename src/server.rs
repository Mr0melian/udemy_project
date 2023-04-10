use std::net::TcpListener;
use std::convert::TryFrom;
use super::http::Request;
use std::io::Read;
pub struct Server{
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        let addr = addr.to_string();
        Server{addr}
    }

    pub fn run(self){
        println!("listing on {}", self.addr);
        let listener = match TcpListener::bind(&self.addr){
            Ok(listener) => listener,
            Err(err) => panic!("can`t be listened by this addr")
        };


        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut bufferr = [0; 1024];
                    match stream.read(&mut bufferr){
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&bufferr));

                           
                            let res = match Request::try_from(&bufferr as &[u8]){
                                Ok(res) =>{}
                                Err(e)=> println!("Feild to parse a request: {}", e),
                            };
                        }
                        Err(e) => println!("Feiled to read from connection:{}", e),
                    }
                }
                Err(e)=> println!("error: {}", e),
            }
        }
    }
}