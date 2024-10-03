use tonic::transport::Channel;
use solana_grpc::geyser_client::GeyserClient;
use solana_grpc::SubscribeUpdate;

pub async fn create_grpc_client(endpoint: &str) -> Result<GeyserClient<Channel>, Box<dyn std::error::Error>> {
    let client = GeyserClient::connect(endpoint).await?;
    Ok(client)
}
