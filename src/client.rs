use std::fmt::Display;
use std::io::{stdin, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct ClientArgs<'a, A>
where
    A: ToSocketAddrs + Display,
{
    proxy_addr: &'a A,
}

impl<'a, A> ClientArgs<'a, A>
where
    A: ToSocketAddrs + Display,
{
    pub fn from_args(cli_args: &'a Vec<A>) -> Self {
        let mut arg_iter = cli_args.into_iter();

        let proxy_addr = arg_iter.next().expect("expected proxy address <addr:port>");

        if let Some(a) = arg_iter.next() {
            panic!("unexpected arg: {}", a)
        }

        Self { proxy_addr }
    }
}

pub struct Client {
    pub proxy_stream: TcpStream,
}

impl Client {
    pub fn new<A>(args: ClientArgs<A>) -> Self
    where
        A: ToSocketAddrs + Display,
    {
        println!("connecting to proxy at: {}", args.proxy_addr);

        //Should add retry logic with exponential falloff
        let proxy_stream =
            TcpStream::connect(args.proxy_addr).expect("server failed to connect to proxy");

        Client { proxy_stream }
    }

    pub fn send_command(&mut self, command: &[u8]) {
        let mut buf = [0_u8; 1024];

        if let Err(_) = self.proxy_stream.write(command) {
            panic!("error occured writing command")
        }
        self.proxy_stream.flush().unwrap();

        if let Err(_) = self.proxy_stream.read(&mut buf) {
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
