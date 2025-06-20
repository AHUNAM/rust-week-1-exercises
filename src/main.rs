use bitcoin::consensus::encode::deserialize;
use bitcoin::Transaction;

fn main() {
    let raw_tx_hex = "0100000001f4b0f8c7898d3c5c4fc05c55504ef89d1e5f635fdf7201ab72580f0f512a4765000000006b483045022100df9fa6e4ac6a7ce9a23cdcfac1c6b0e73ab65d8657be92d74a4a61b4f1a7fcb302207278c86302a14e9d78ec93e24ab329cb258d48da55e260d3c9377f6adf81e2da012102c5a44c8e88d9945d4f8b0c6aa1224495e68a5d17460ec1ac5eb93e2957a64f59ffffffff01e8030000000000001976a914b1f2d3f3ecbfb9ac4d6cf9df8f05e5bcbf2e4dfc88ac00000000";
    let raw_tx_bytes = hex::decode(raw_tx_hex).expect("Invalid hex string");
    // Deserialize the raw transaction bytes into a Transaction object
    let tx: Transaction = deserialize(&raw_tx_bytes).expect("Failed to decode transaction");

    // Print transaction details
    println!("Transaction ID: {}", tx.compute_txid());
    println!("Version: {}", tx.version);
    println!("Lock Time: {}", tx.lock_time);
    println!("Inputs: {:?}", tx.input);
    println!("Outputs: {:?}", tx.output);
}
