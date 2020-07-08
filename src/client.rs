use std::fs;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;

use quinn::Endpoint;

mod row;

use row::RowGenerator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }
}

fn get_certificate_authority() -> Result<quinn::Certificate> {
    // Get the local directory where the certificate is stored
    let dirs = directories_next::ProjectDirs::from("org", "sybl", "quinn").unwrap();
    let path = dirs.data_local_dir().join("cert.der");

    // Read the certificate from the file and parse it
    let cert = fs::read(path)?;
    Ok(quinn::Certificate::from_der(&cert)?)
}

#[tokio::main]
async fn run() -> Result<()> {
    // Build the config and add the self-signed certificate
    let mut client_config = quinn::ClientConfigBuilder::default();
    client_config.add_certificate_authority(get_certificate_authority()?)?;

    // Begin building an Endpoint and add the config
    let mut builder = Endpoint::builder();
    builder.default_client_config(client_config.build());

    // Bind the client to send from port 6000
    let host = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:6000")?);
    let (endpoint, _) = builder.bind(&host)?;

    // Configure the server information and connect to it
    let hostname = "cinnamon";
    let server = SocketAddr::V4(SocketAddrV4::from_str("127.0.0.1:5000")?);
    let new_connection = endpoint.connect(&server, &hostname)?.await?;

    // Get the actual connection and setup bidirectional channels
    let quinn::NewConnection { connection, .. } = new_connection;
    let (mut send, _) = connection.open_bi().await?;

    let generator = RowGenerator::new(1000);

    for row in generator {
        let message = bincode::serialize(&row)?;
        send.write_all(&message).await?;
    }

    send.finish().await?;

    Ok(())
}
