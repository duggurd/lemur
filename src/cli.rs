use crate::client::{Client, ClientArgs};
use crate::proxy::{Proxy, ProxyArgs};
use crate::server::{Server, ServerArgs};
use std::env::args;

enum Target {
    Server,
    Client,
    Proxy,
}

impl Target {
    fn from_str(data: String) -> Self {
        match data.to_lowercase().as_str() {
            "client" => Self::Client,
            "proxy" => Self::Proxy,
            "server" => Self::Server,
            t => panic!("invalid target: {t}, should be one of 'client'|'proxy'|'server'"),
        }
    }
}

struct Command {
    target: Target,
    cli_args: Vec<String>,
}

impl Command {
    fn from_args() -> Self {
        let mut args = args();

        let _ = args.next().unwrap();

        let target = Target::from_str(
            args.next()
                .expect("missing target: 'client'|'proxy'|'server'"),
        );

        let cli_args = args.collect::<Vec<String>>();

        Self { target, cli_args }
    }
}

pub struct Cli {
    cmd: Command,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            cmd: Command::from_args(),
        }
    }

    pub fn run(&mut self) {
        match self.cmd.target {
            Target::Client => {
                let client_args = ClientArgs::from_args(&self.cmd.cli_args);
                let mut client = Client::new(client_args);
                client.run()
            }
            Target::Proxy => {
                let proxy_args = ProxyArgs::from_args(&self.cmd.cli_args);
                let mut proxy = Proxy::new(proxy_args);
                proxy.run()
            }
            Target::Server => {
                let server_args = ServerArgs::from_args(&self.cmd.cli_args);
                let mut server = Server::new(server_args);
                server.run();
            }
        }
    }
}
