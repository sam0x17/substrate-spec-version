use subxt::{OnlineClient, SubstrateConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Provide the WebSocket URL directly when creating the client
    let url = "wss://test.chain.opentensor.ai:443"; // replace with the actual URL as needed
    let client = OnlineClient::<SubstrateConfig>::from_url(url).await?;

    // Attempting to access runtime version directly from the backend or similar method
    let runtime_version = client.runtime_version();

    // Print the spec version
    println!("Spec Version: {}", runtime_version.spec_version);

    Ok(())
}
