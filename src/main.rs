use std::net::TcpListener;
use std::{env::args, io::Write};

fn main() {
    let argv = args().collect::<Vec<String>>();

    let server = TcpListener::bind(argv.get(1).unwrap_or(&"127.0.0.1".to_string())).unwrap();

    for req in server.incoming() {
        if req.is_ok() {
            let mut tcpreq = req.unwrap();
            tcpreq
                .write_all(b"Hello From Crollar!")
                .expect("Couldn't write to request");
        }
    }
}
