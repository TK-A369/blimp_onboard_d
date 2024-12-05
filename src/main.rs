use tokio;

use blimp_comm;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let channels = vec![blimp_comm::CommChannel::Server(core::net::SocketAddr::new(
        std::net::IpAddr::V4(core::net::Ipv4Addr::new(127, 0, 0, 1)),
        9898,
    ))];
    blimp_comm::start_communication(channels).await;

    tokio::signal::ctrl_c().await.unwrap();
}
