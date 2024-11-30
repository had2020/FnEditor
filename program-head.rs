use std::process::Command; // terimal commands

// Server script
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn sh_command1(command:&str) {
    let output = Command::new("sh")
    .arg("-c")
    .arg(command)
    .output()
    .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    // norm
    /*
    let output = Command::new("echo")
        .arg("Hello, world!")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    */

    // shell syntax
    /* 
    let output = Command::new("sh")
    .arg("-c")
    .arg("echo Hello, world!")
    .output()
    .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    */

    sh_command1("echo running");
    //sh_command1("mkdir test1"); any sh command works
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buffer));
    }

}