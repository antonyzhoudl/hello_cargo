use solana_client::rpc_client::RpcClient;
use solana_sdk::{feature_set::tx_wide_compute_cap, signature::Signature};
use solana_sdk::transaction::Transaction;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_transaction_status::{EncodedTransaction, UiTransactionEncoding, EncodedConfirmedTransactionWithStatusMeta};

fn main() {
    println!("Hello, world!");
    let rpc_client = RpcClient::new_with_commitment(
        "127.0.0.1:10001".to_string(),
        solana_sdk::commitment_config::CommitmentConfig::confirmed()
    );

    // 提供待查询的交易签名
    let signature_str = "2YYKryjHdNuvV1vkECmEHqE3XxPLwp2EhGH3GpAqZmdcv2pLT1BjoQb75QgVU7LZDB5pubqBdiWk9XSCPk1ZDrRU"; // 用实际的交易签名替换
    let signature = signature_str.parse::<Signature>().expect("Invalid signature format");

    // 设置请求配置，以Base64格式获取详细信息
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Base64),
        ..Default::default()
    };

    let  tx = rpc_client.get_transaction_with_config(&signature, config);

    // 获取交易详情
    match tx {
        Ok(transaction)=> {
            println!("Transaction details: {:?}", transaction);
            // let jtx = transaction.transaction;
        },
        Err(err) => println!("Error fetching transaction: {:?}", err),
    }
}