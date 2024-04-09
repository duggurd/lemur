mod client;
mod proxy;
mod server;

mod cli;
use cli::Cli;

fn main() {
    let mut cli = Cli::new();
    cli.run();
}
