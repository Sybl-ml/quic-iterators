use std::fs;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;

use futures::StreamExt;
use quinn::Endpoint;

mod row;

use row::Row;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }
}

fn get_certificate_chain_and_key() -> Result<(quinn::CertificateChain, quinn::PrivateKey)> {
    // Get the local directory to store the certificate and key
    let dirs = directories_next::ProjectDirs::from("org", "sybl", "quinn").unwrap();
    let path = dirs.data_local_dir();

    // Get the filepaths for the certificate and key
    let cert_path = path.join("cert.der");
    let key_path = path.join("key.der");

    // Generate a self-signed certificate
    let cert = rcgen::generate_simple_self_signed(vec!["cinnamon".into()]).unwrap();

    // Extract the key and certificate
    let key = cert.serialize_private_key_der();
    let cert = cert.serialize_der().unwrap();

    // Ensure the path exists and write the two files
    fs::create_dir_all(&path)?;
    fs::write(&cert_path, &cert)?;
    fs::write(&key_path, &key)?;

    // Put the key and certificate into quinn form
    let key = quinn::PrivateKey::from_der(&key)?;
    let cert = quinn::Certificate::from_der(&cert)?;

    Ok((quinn::CertificateChain::from_certs(vec![cert]), key))
}

#[tokio::main]
async fn run() -> Result<()> {
    // Ban the use of unidirectional streams
    let mut transport_config = quinn::TransportConfig::default();
    transport_config.stream_window_uni(0);

    // Build the server configuration
    let mut server_config = quinn::ServerConfig::default();
    server_config.transport = Arc::new(transport_config);
    let mut server_config = quinn::ServerConfigBuilder::new(server_config);

    // Add the certificate chain and respective key
    let (cert_chain, key) = get_certificate_chain_and_key()?;
    server_config.certificate(cert_chain, key)?;

    // Build the Endpoint and allow listening for incoming connections
    let mut builder = Endpoint::builder();
    builder.listen(server_config.build());

    // Allow it to listen on port 5000
    let host = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:5000")?);
    let (_, mut incoming) = builder.bind(&host)?;

    // For each new incoming connection
    while let Some(conn) = incoming.next().await {
        // Get the stream of bidirectional streams it can create
        let quinn::NewConnection { mut bi_streams, .. } = conn.await?;

        println!("Encountered a new connection");

        // For every stream it creates
        while let Some(stream) = bi_streams.next().await {
            // Get the respective channels
            let (mut send, mut recv) = stream?;

            // Allocate a buffer
            let mut buffer = [0 as u8; 1024];

            while let Some(size) = recv.read(&mut buffer).await? {
                let message: Row = bincode::deserialize(&buffer[..size]).unwrap();
                println!("Received '{:?}' from the client", message);
            }

            // Respond to the client
            send.write_all(b"Thanks").await?;
            send.finish().await?;
        }
    }

    Ok(())
}
