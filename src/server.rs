use std::net::TcpListener;
use std::io::Read;
// Definition of server structure
pub struct Server {
    addr: String
}

// Defining methods on struct server
impl Server {

    // new function initilizes new server and receives addr as input parameter
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    // run method starts the http server and keeps looping forever
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer))
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish connection: {}", e),
                _ => {} // This acts like default condition and hence catches all
            }
        }
    }
}

