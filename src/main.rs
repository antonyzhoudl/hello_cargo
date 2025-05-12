use anyhow::Result;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status_client_types::UiTransactionEncoding;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("http://127.0.0.1:10001"),
        CommitmentConfig::confirmed(),
    );

    let tx_sig = Signature::from_str(
        "k263vsY8CNmAmMxynQdsfgP9sqyKJhSUfnd9CFJbtBCASSiKKWeaQd9MaA5mkVpotAKRXUaaYazoigZ3CZve67R",
    )?;

    let config = RpcTransactionConfig {
        commitment: CommitmentConfig::finalized().into(),
        encoding: UiTransactionEncoding::Base64.into(),
        max_supported_transaction_version: Some(0),
    };

    let transaction = client.get_transaction_with_config(&tx_sig, config).await?;

    println!("{:#?}", transaction);

    Ok(())
}