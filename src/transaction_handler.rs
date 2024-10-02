// src/transaction_handler.rs

use yellowstone_grpc_proto::prelude::{SubscribeUpdateTransaction, TransactionStatus}; // Adjust imports as necessary
use log::{info, error};

pub async fn handle_transaction_update(update: SubscribeUpdateTransaction) {
    // Check if the transaction is pending
    if is_transaction_pending(&update) {
        info!("Pending transaction detected: {:?}", update);
    } else {
        info!("Transaction status: {:?}", update.status);
    }
}

fn is_transaction_pending(update: &SubscribeUpdateTransaction) -> bool {
    // Check the transaction status
    match update.status {
        TransactionStatus::Pending => true,
        _ => false,
    }
}
