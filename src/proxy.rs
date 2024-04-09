use std::fmt::Display;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct ProxyArgs<'a, A>
where
    A: ToSocketAddrs + Display,
{
    server_l_addr: &'a A,
    client_l_addr: &'a A,
}

impl<'a, A> ProxyArgs<'a, A>
where
    A: ToSocketAddrs + Display,
{
    pub fn from_args(cli_args: &'a Vec<A>) -> Self {
        let mut arg_iter = cli_args.into_iter();

        let server_l_addr = arg_iter
            .next()
            .expect("expected server listen address <addr:port>");

        let client_l_addr = arg_iter
            .next()
            .expect("expected client listen address <addr:port>");

        if let Some(a) = arg_iter.next() {
            panic!("unexpected arg: {}", a)
        }

        Self {
            server_l_addr,
            client_l_addr,
        }
    }
}

pub struct Proxy {
    server_stream: TcpStream,
    client_stream: TcpStream,
}

impl Proxy {
    /// Wait for both endpoints to connect
    pub fn new<A>(args: ProxyArgs<A>) -> Self
    where
        A: ToSocketAddrs + Display,
    {
        println!("Waiting for peer connections...");

        let server_listener = TcpListener::bind(args.server_l_addr).unwrap();
        let client_listener = TcpListener::bind(args.client_l_addr).unwrap();

        server_listener.set_nonblocking(true).unwrap();
        client_listener.set_nonblocking(true).unwrap();

        let mut client_stream: Option<TcpStream> = None;
        let mut server_stream: Option<TcpStream> = None;

        while client_stream.is_none() | server_stream.is_none() {
            if server_stream.is_none() {
                match server_listener.accept() {
                    Ok((s, _)) => {
                        s.set_nonblocking(false).unwrap();
                        server_stream = Some(s);
                        println!("server connected");
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                    Err(_) => panic!("failed to accept server connection"),
                }
            }

            if client_stream.is_none() {
                match client_listener.accept() {
                    Ok((s, _)) => {
                        s.set_nonblocking(false).unwrap();
                        client_stream = Some(s);
                        println!("client connected");
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                    Err(_) => panic!("failed to accept client connection"),
                }
            }
        }

        Self {
            client_stream: client_stream.unwrap(),
            server_stream: server_stream.unwrap(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut buf = [0_u8; 1024];

            if let Err(_) = self.client_stream.read(&mut buf) {
                println!("error reading");
                self.client_stream.write(b"failed to read").unwrap();
                self.client_stream.flush().unwrap();
            }

            self.server_stream.write(&buf).unwrap();
            self.server_stream.flush().unwrap();

            buf.fill(0);

            self.server_stream.read(&mut buf).unwrap();
            self.client_stream.write(&buf).unwrap();
            self.client_stream.flush().unwrap();
        }
    }
}
