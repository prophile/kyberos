use clap::Parser;
use reqwest::Client;
use std::fmt::{self, Display, Formatter};
use kyberos_protocol::SystemInfo;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "kyctl: A command-line interface for Kyberos.")]
struct Args {
    /// The address of the server
    #[arg(short, long, default_value = "http://localhost:15333")]
    server: String,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "server: {}", self.server)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let client = Client::new();

    let url = format!("{}/server/version", args.server);
    let response = client.get(&url).send().await.unwrap();
    let info = response.json::<SystemInfo>().await.unwrap();

    println!("Server version: {}", info.server_version);
}
// fn main() {
//     let args = Args::parse();

//     println!("kyctl: {}", args);
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
