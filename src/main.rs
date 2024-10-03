// src/main.rs

use crate::grpc_client::create_grpc_client;
use crate::transaction_handler::handle_transaction_update;
use yellowstone_grpc_proto::prelude::{SubscribeRequest, SubscribeUpdate};
use futures::stream::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let endpoint = env::var("SOLANA_GRPC_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:10000".to_string());
    let mut client = create_grpc_client(&endpoint).await?;

    // Create a subscription request for transactions
    let request = SubscribeRequest {
        transactions: HashMap::new(), // Fill in with appropriate filters if needed
        ..Default::default()
    };

    let mut stream = client.subscribe_with_request(request).await?.into_inner();

    while let Some(update) = stream.next().await {
        match update {
            Ok(SubscribeUpdate::Transaction(tx)) => {
                handle_transaction_update(tx).await;
            }
            Err(e) => {
                error!("Error receiving update: {:?}", e);
            }
            _ => {}
        }
    }

    Ok(())
}
