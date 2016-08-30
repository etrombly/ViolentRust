extern crate pwhash;

use pwhash::unix_crypt;
use std::fs::File;
use std::io::Read;

fn read_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut f = try!(File::open(path));
    let mut lines = String::new();
    try!(f.read_to_string(&mut lines));
    Ok(lines.lines().map(|s| s.to_owned()).collect())
}

fn main() {
    let lines = read_lines("passwords.txt").unwrap();
    for line in lines{
        let split: Vec<&str> = line.split(':').collect();
        if split.len() > 1 {
            println!("[*] Cracking Password For: {}", split[0]);
            test_pass(split[1]);
        }
    }
}

fn test_pass (crypt_pass: &str) {
    let lines = read_lines("dictionary.txt").unwrap();
    for word in lines {
        if unix_crypt::verify(&word, crypt_pass) {
            println!("[+] Found Password: {}", word);
            return
        }
    }
    println!("[-] Password not found");
}
