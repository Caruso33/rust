use http::request::Request;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(self) -> () {
            println!("Running server on {}", &self.addr);
        }
    }
}
mod http {
    pub mod request {
        pub enum Method {
            GET,
            POST,
            PUT,
            PATCH,
            DELETE,
            OPTIONS,
            HEAD,
            CONNECT,
            TRACE,
        }
    }

    pub mod method {
        use super::request;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: request::Method,
        }
    }
}
