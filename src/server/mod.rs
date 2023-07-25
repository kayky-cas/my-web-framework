mod request;
use std::{collections::HashMap, io, net::TcpListener};

trait Server {
    fn run(&self) -> io::Result<()>;
}

struct WebServer {
    address: (String, u16),
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
