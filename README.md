# Solana Pending Transaction Finder

A Rust application that finds pending and unprocessed transactions from a Solana node using the Yellowstone gRPC client and Jito relayer.

## Features

- Connects to a Solana node via gRPC.
- Subscribes to transaction updates.
- Filters and logs pending and unprocessed transactions.
- Integrates with Jito relayer for enhanced transaction processing.

## Requirements

- [Rust](https://www.rust-lang.org/) (version 1.50 or higher)
- A running Solana node with Yellowstone gRPC enabled.
- Jito relayer setup and configuration.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/solana_pending_transaction_finder.git
   cd solana_pending_transaction_finder
   ```