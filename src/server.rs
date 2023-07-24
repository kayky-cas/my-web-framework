use std::{io, net::TcpListener};

struct WebServer {
    address: (String, u16),
}

trait Server {
    fn run(&self) -> io::Result<()>;
}

impl Server for WebServer {
    fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;

        for stream in listener.incoming() {
            let _stream = stream?;
        }

        Ok(())
    }
}
