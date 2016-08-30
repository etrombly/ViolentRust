use std::env;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn read_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut f = try!(File::open(path));
    let mut lines = String::new();
    try!(f.read_to_string(&mut lines));
    Ok(lines.lines().map(|s| s.to_owned()).collect())
}

fn ret_banner(socket_addr: SocketAddr) -> Result<[u8; 1024], ()> {
    if let Ok(mut stream) = TcpStream::connect(&socket_addr){
        let mut data = [0; 1024];

        //added this because original doesn't actually grab http(s) banners
        if socket_addr.port() == 80 || socket_addr.port() == 443 {
            stream.write(b"HEAD / HTTP/1.0\n\n").unwrap();
        }

        let mut handle = stream.take(1024);
        if let Ok(_) = handle.read(&mut data){
            return Ok(data)
        }
    }
    Err(())
}

fn check_vulns(banner: &str, filename: &str) {
    lines = read_lines(filename);
    for line in lines {
        if banner.contains(line) {
            println!("[+] Server is vulnerable: {}", line);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[-] Usage: {} <vuln filename>", args[0]);
        return
    }
    let port_list: Vec<u16> = vec![21,22,25,80,110,443];
    for x in 35..37 {
        let ip = IpAddr::V4(Ipv4Addr::new(10,0,0,x));
        for port in &port_list {
            let socket_addr = SocketAddr::new(ip, *port);
            if let Ok(banner) = ret_banner(socket_addr){
                println!("[+] {}:{}\n{}", ip, port, String::from_utf8_lossy(&banner));
                check_vulns(banner, args[1]);
            } else {
                println!("[-] {}:{} couldn't connect",ip,port);
            }
        }
    }
}
