use anyhow::Result;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status_client_types::{UiTransactionEncoding, UiTransactionTokenBalance, option_serializer::OptionSerializer};
use std::{iter::Skip, str::FromStr};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("http://127.0.0.1:10001"),
        CommitmentConfig::confirmed(),
    );

    let tx_sig = Signature::from_str(
        "2cmH8NGBeP9ivNAKY6HKMPv94ajUsR1vSvPyuHE432x9cxWSE1Jeacn7GQB7GKm3BLbJhEMtMw7XDCn7UDeTvbfV",
    )?;

    let config = RpcTransactionConfig {
        commitment: CommitmentConfig::finalized().into(),
        encoding: UiTransactionEncoding::Base64.into(),
        max_supported_transaction_version: Some(0),
    };

    let transaction = client.get_transaction_with_config(&tx_sig, config).await;

    match  transaction {
        Ok(txn) =>{
            let tp = txn.transaction.transaction.decode();
            if let Some(tpp) = tp {
                let acts =  tpp.message.static_account_keys();
                if let Some(meta) = txn.transaction.meta{
                    let dft:Vec<UiTransactionTokenBalance> = vec![];
                    let token_balances= meta.post_token_balances.unwrap_or(dft);
                    for (index, balance) in token_balances.iter().enumerate() {
                        println!("Index: {} \n account: {}  \n  amount:{}", index, acts[index].to_string(),meta.post_balances[index] - meta.pre_balances[index]);
                        println!("Mint: {}", balance.mint);
                        println!("Amount: {}", balance.ui_token_amount.real_number_string());
                    }
                }

            }
           
        }
        Err(err) => {
            println!("txn receiver error: {}",err)
        }
    }

    Ok(())
}

/*
    match transaction_result {
        Ok(Some(transaction)) => {
            // Here, we correctly handle the presence of a transaction
            println!("Transaction Details: {:?}", transaction);
            if let Some(meta) = &transaction.meta {
                if let Some(pre_balances) = &meta.pre_balances {
                    if let Some(post_balances) = &meta.post_balances {
                        for (i, (pre, post)) in pre_balances.iter().zip(post_balances.iter()).enumerate() {
                            let account_key = &transaction.transaction.message.account_keys[i];
                            let balance_change = post - pre;
                            println!("Account: {:?} Balance Change: {}", account_key, balance_change);
                        }
                    }
                }
            }
        },
        Ok(None) => {
            println!("Transaction not found.");
        },
        Err(err) => {
            eprintln!("An error occurred: {}", err);
        },
    }
*/