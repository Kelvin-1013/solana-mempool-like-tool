use solana_grpc::SubscribeUpdate;
use solana_grpc::TransactionStatus; // Adjust the import based on your actual structure

pub fn is_pending(update: &SubscribeUpdate) -> bool {
    match &update {
        SubscribeUpdate::Transaction(tx_update) => {
            // Check if the transaction status is pending
            if let Some(status) = &tx_update.status {
                match status {
                    TransactionStatus::Pending => true,
                    _ => false,
                }
            } else {
                false // If there's no status, consider it not pending
            }
        },
        _ => false, // If the update is not a transaction update, it's not pending
    }
}
