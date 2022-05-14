pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) -> {
        println!("Listen on addr: {}", self.addr);

        std::result::Result
        let listener = TcpListener::bind(self.addr);

        //create try catch on 'listener' to check connection
        match listener {
            Ok(lt) => {}
            Err(e) => println!("{}", e),
        }
    }
}