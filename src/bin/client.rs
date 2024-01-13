use tokio::net::TcpStream;



#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse our URL...
    let url = "http://127.0.0.1:3000".parse::<hyper::Uri>()?;

    // Get the host and the port
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    // Open a TCP connection to the remote host
    let stream = TcpStream::connect(address).await?;

    //let _ = stream.try_read(&mut [0; 128]);

    std::future::pending::<()>().await;

    Ok(())
}
