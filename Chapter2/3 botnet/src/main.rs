extern crate ssh2;

use ssh2::Session;
use std::net::TcpStream;
use std::io::Read;

struct Client<'a> {
    host: &'a str,
    stream: TcpStream,
    session: Session,
}

impl<'a> Client<'a>{
    fn new (host: &'a str, user: &str, password: &str) -> Client<'a> {
        let tcp = TcpStream::connect((host, 22)).unwrap();
        let mut session = Session::new().unwrap();
        session.handshake(&tcp).unwrap();

        session.userauth_password(user, password).unwrap();

        Client {
            host: host,
            stream: tcp,
            session: session,
        }

    }

    fn send_command (&self, command: &str) -> String {
        let mut channel = self.session.channel_session().unwrap();
        channel.exec(command).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        s
    }
}

fn botnet_command(botnet: &[Client], command: &str){
    for bot in botnet {
        println!("[*] Output from {}", bot.host);
        println!("[+] {}", bot.send_command(command));
    }
}

fn main() {
    let mut botnet = vec![];
    botnet.push(Client::new("127.0.0.1", "root", "toor"));
    botnet.push(Client::new("127.0.0.1", "root", "toor"));
    botnet.push(Client::new("127.0.0.1", "root", "toor"));
    botnet_command(&botnet, "uname -v");
    botnet_command(&botnet, "cat /etc/issue");
}
