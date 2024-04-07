use std::env::args;
use std::fmt::Display;
use std::io::{stdin, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn new<A>(server_address: A) -> Self
    where
        A: ToSocketAddrs + Display,
    {
        println!("connecting to server: {}", server_address);

        let stream = match TcpStream::connect(server_address) {
            Ok(s) => s,
            Err(_) => panic!("failed to bind to server, is the server running?"),
        };

        Client { stream: stream }
    }

    pub fn from_args() -> Self {
        let mut args = args();
        let _ = args.next().unwrap();

        let addr = match args.next() {
            Some(a) => a,
            None => panic!("missing address <ip:port>"),
        };

        Self {
            stream: TcpStream::connect(addr).unwrap(),
        }
    }

    pub fn send_command(&mut self, command: &[u8]) {
        let mut buf = [0_u8; 1024];

        if let Err(e) = self.stream.write(command) {
            panic!("error occured writing command")
        }
        self.stream.flush().unwrap();

        if let Err(e) = self.stream.read(&mut buf) {
            panic!("Error occured while reading from stream")
        }

        let resp = String::from_utf8(buf.to_vec()).unwrap();

        println!("{}", resp);
    }

    pub fn run(&mut self) {
        let mut stdin = stdin();

        loop {
            let mut buf = [0_u8; 1024];
            if let Err(_) = stdin.read(&mut buf) {
                panic!("Failed to read stdin");
            }

            self.send_command(&buf);
        }
    }
}

pub struct Server {
    pub listener: TcpListener,
}

impl Server {
    pub fn new<A>(listen_address: A) -> Self
    where
        A: ToSocketAddrs + Display,
    {
        println!("binding listener to: {}", listen_address);
        let listener = match TcpListener::bind(listen_address) {
            Ok(l) => l,
            Err(_) => panic!("Failed to bind listener"),
        };

        Server { listener: listener }
    }

    pub fn from_args() -> Self {
        let mut args = args();
        let _ = args.next().unwrap();

        let addr = match args.next() {
            Some(a) => a,
            None => panic!("missing address <ip:port>"),
        };

        Self {
            listener: TcpListener::bind(addr).unwrap(),
        }
    }

    pub fn run(&mut self) {
        let mut buf = [0_u8; 1024];

        let (mut stream, _) = match self.listener.accept() {
            Ok(s) => s,
            Err(e) => panic!("error occured when accepting stream"),
        };

        loop {
            if let Err(e) = stream.read(&mut buf) {
                panic!("error occured when reading stream");
            }

            let cmd = String::from_utf8(buf.to_vec()).unwrap();

            println!("got command: {}", cmd);

            let output = self.execute_command(&cmd);

            stream.write(output.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    fn execute_command(&mut self, command: &str) -> String {
        let cleaned_command = command.replace('\0', "");

        // currently creates new shell session on every run, not what i want

        #[cfg(target_os = "windows")]
        let output = Command::new("cmd")
            .arg("/c")
            .arg(cleaned_command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        #[cfg(target_os = "linux")]
        let output = Command::new("sh")
            .arg("-c")
            .arg(cleaned_command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        let mut data = String::from_utf8(output.stdout).unwrap();
        if data == "" {
            data = format!("{}", output.status)
        }
        println!("command output: {}", data);

        return data;
    }
}

pub struct Mitm {
    listener: TcpListener,
    server_stream: TcpStream,
}

impl Mitm {
    pub fn new<A>(listen_addr: A, server_addr: A) -> Self
    where
        A: ToSocketAddrs + Display,
    {
        let server_stream: TcpStream = loop {
            println!("waiting for target server to be up");
            match TcpStream::connect(&server_addr) {
                Err(_) => sleep(Duration::from_secs(1)),
                Ok(s) => break s,
            }
        };

        println!("Connected to server!");

        Mitm {
            listener: TcpListener::bind(listen_addr).unwrap(),
            server_stream: server_stream,
        }
    }
    pub fn from_args() -> Self {
        let mut args = args();
        let _ = args.next().unwrap();

        let listen_addr = match args.next() {
            Some(a) => a,
            None => panic!("missing listening address <ip:port>"),
        };

        let server_addr = match args.next() {
            Some(a) => a,
            None => panic!("missing server address <ip:port>"),
        };

        Mitm::new(listen_addr, server_addr)
    }

    pub fn run(&mut self) {
        let (mut client_stream, _) = self.listener.accept().unwrap();

        loop {
            let mut buf = [0_u8; 1024];

            if let Err(_) = client_stream.read(&mut buf) {
                println!("error reading");
                client_stream.write(b"failed to read").unwrap();
                client_stream.flush().unwrap();
            }

            self.server_stream.write(&buf).unwrap();
            self.server_stream.flush().unwrap();

            buf.fill(0);

            self.server_stream.read(&mut buf).unwrap();
            client_stream.write(&buf).unwrap();
            client_stream.flush().unwrap();
        }
    }
}
