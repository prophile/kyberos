use kyberos_protocol::SystemInfo;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use warp::Filter;

const PORT: u16 = 15333;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let info = warp::path!("server" / "version").map(|| {
        warp::reply::json(&SystemInfo {
            server_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    });

    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), PORT);

    warp::serve(info).run(addr).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
