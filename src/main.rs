use structopt::StructOpt;
use subxt::{OnlineClient, SubstrateConfig};

/// Command-line arguments configuration using structopt
#[derive(StructOpt, Debug)]
struct Cli {
    /// WebSocket URL of the Substrate node
    #[structopt(name = "URL")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Cli::from_args();

    // Create a new API client using the provided URL
    let client = OnlineClient::<SubstrateConfig>::from_url(&args.url).await?;

    // Fetch the runtime version
    let runtime_version = client.runtime_version();

    // Output only the spec version
    println!("{}", runtime_version.spec_version);

    Ok(())
}
