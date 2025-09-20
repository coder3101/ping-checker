use std::{net::IpAddr, time::Duration};

use axum::{Router, extract::Path, http::StatusCode, routing::get};
use ping_async::IcmpEchoRequestor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| String::from("0.0.0.0:8088"));
    let app = Router::new().route("/ping/{ip}", get(perform_ping));

    tracing::info!("Listening on {listen_addr}");

    let listener = tokio::net::TcpListener::bind(listen_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn perform_ping(Path(ip): Path<String>) -> Result<(), StatusCode> {
    let ip: IpAddr = ip.parse().map_err(|e| {
        tracing::warn!(err=%e, "perform_ping: invalid IP address");
        StatusCode::BAD_REQUEST
    })?;

    tracing::debug!("Pinging {ip}");

    let pp = IcmpEchoRequestor::new(ip, None, None, Some(Duration::from_secs(2))).map_err(|e| {
        tracing::warn!(err=?e, "perform_ping: failed to create ping requestor");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Err(e) = pp.send().await {
        tracing::warn!(err=?e, "perform_ping: ping failed");
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    Ok(())
}
