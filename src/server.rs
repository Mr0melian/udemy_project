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
    }
}