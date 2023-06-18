
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
        println!("Listening on {}", self.addr)
    }
}

