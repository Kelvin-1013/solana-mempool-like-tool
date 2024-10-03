use tonic::transport::Channel;
use solana_grpc::geyser_client::GeyserClient;
use solana_grpc::SubscribeUpdate;

pub async fn create_grpc_client(endpoint: &str) -> Result<GeyserClient<Channel>, Box<dyn std::error::Error>> {

    let endpoint = tonic::transport::Endpoint::from_shared(endpoint.to_string())?;
    let channel = endpoint.connect().await?;
    let client = GeyserClient::new(channel);
    Ok(client)
}