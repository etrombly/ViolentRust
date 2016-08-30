extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::net;
use std::thread;
use std::io::{Read, Write};

const USAGE: &'static str = "
Port Scan

Usage:
  port_scan <host> <ports>...
  port_scan (-h | --help)

Options:
  -h --help   show this screen
  host        target host
  ports        target ports
";

#[derive (Debug, RustcDecodable)]
struct Args {
    arg_host: String,
    arg_ports: Vec<u16>,
}

fn conn_scan (host: String, port: &u16) {
    let host: &str = &host;
    let mut data = [0; 100];

    if let Ok(mut stream) = net::TcpStream::connect((host, *port)) {
        stream.write(b"ViolentPython\r\n").unwrap();
        let mut handle = stream.take(100);
        if let Ok(_) = handle.read(&mut data) {
            println!("[+] {} tcp open\n[+] {}", port, String::from_utf8_lossy(&data));
        }
    } else {
        println!("[-] {} tcp closed", port);
    }
}

fn port_scan (host: String, ports: Vec<u16>) {
    let mut children = vec![];

    for port in ports {
        let host = host.clone();
        children.push(thread::spawn(move || {
            conn_scan(host, &port);
        }));
    }
    for child in children {
        let _ = child.join();
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                     .and_then(|d| d.decode())
                     .unwrap_or_else(|e| e.exit());
    port_scan(args.arg_host, args.arg_ports);
}
