use crate::grpc_client::GeyserClient;
use solana_grpc::SubscribeUpdate;
use log::{info, error};

pub async fn subscribe_to_transactions(client: &mut GeyserClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = client.subscribe_updates().await?.into_inner();

    while let Some(update) = stream.message().await? {
        handle_transaction_update(update);
    }

    Ok(())
}

pub fn handle_transaction_update(update: SubscribeUpdate) {
    // Process the update
    info!("Received transaction update: {:?}", update);

    // Here you can filter for pending and unprocessed transactions
    // For example:
    // if is_pending(&update) {
    //     info!("Pending transaction detected: {:?}", update);
    // }
}
