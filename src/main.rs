fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

// Definition of server structure
struct Server {
    addr: String
}

// Defining methods on struct server
impl Server {

    // new function initilizes new server and receives addr as input parameter
    fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    // run method starts the http server and keeps looping forever
    fn run(self) {
        println!("Listening on {}", self.addr)
    }
}

