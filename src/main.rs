use solana_pending_transaction_finder::{create_grpc_client, subscribe_to_transactions};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let endpoint = env::var("SOLANA_GRPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:50051".to_string());

    let mut client = create_grpc_client(&endpoint).await.expect("Failed to create gRPC client");

    subscribe_to_transactions(&mut client).await.expect("Failed to subscribe to transactions");
}
