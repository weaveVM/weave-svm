//! PayTube's custom transaction format, tailored specifically for SOL or SPL
//! token transfers.
//!
//! Mostly for demonstration purposes, to show how projects may use completely
//! different transactions in their protocol, then convert the resulting state
//! transitions into the necessary transactions for the base chain - in this
//! case Solana.

use {
    solana_sdk::{
        instruction::Instruction as SolanaInstruction,
        pubkey::Pubkey,
        system_instruction,
        transaction::{
            SanitizedTransaction as SolanaSanitizedTransaction, Transaction as SolanaTransaction,
        },
    },
    spl_associated_token_account::get_associated_token_address,
    std::collections::HashSet,
};

// WeaveVM imports

use crate::utils::{get_env_var, WVM_DATA_SETTLER};
use abi::Address;
use ethers::types::H256;
use ethers::utils::hex;
use ethers::{prelude::*, utils};
use ethers_providers::{Http, Provider};
use std::str::FromStr;
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

/// A simple PayTube transaction. Transfers SPL tokens or SOL from one account
/// to another.
///
/// A `None` value for `mint` represents native SOL.
///
#[derive(Debug)]
pub struct PayTubeTransaction {
    pub mint: Option<Pubkey>,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

impl From<&PayTubeTransaction> for SolanaInstruction {
    fn from(value: &PayTubeTransaction) -> Self {
        let PayTubeTransaction {
            mint,
            from,
            to,
            amount,
        } = value;
        if let Some(mint) = mint {
            let source_pubkey = get_associated_token_address(from, mint);
            let destination_pubkey = get_associated_token_address(to, mint);
            return spl_token::instruction::transfer(
                &spl_token::id(),
                &source_pubkey,
                &destination_pubkey,
                from,
                &[],
                *amount,
            )
            .unwrap();
        }
        system_instruction::transfer(from, to, *amount)
    }
}

impl From<&PayTubeTransaction> for SolanaTransaction {
    fn from(value: &PayTubeTransaction) -> Self {
        SolanaTransaction::new_with_payer(&[SolanaInstruction::from(value)], Some(&value.from))
    }
}

impl From<&PayTubeTransaction> for SolanaSanitizedTransaction {
    fn from(value: &PayTubeTransaction) -> Self {
        SolanaSanitizedTransaction::try_from_legacy_transaction(
            SolanaTransaction::from(value),
            &HashSet::new(),
        )
        .unwrap()
    }
}

/// Create a batch of Solana transactions, for the Solana SVM's transaction
/// processor, from a batch of PayTube instructions.
pub fn create_svm_transactions(
    paytube_transactions: &[PayTubeTransaction],
) -> Vec<SolanaSanitizedTransaction> {
    paytube_transactions
        .iter()
        .map(SolanaSanitizedTransaction::from)
        .collect()
}

/// WeaveVM Transaction utils

pub async fn propagate_wvm_transaction(
    client: &Client,
    address_from: &Address,
    address_to: &Address,
    data: Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {
    // 2.14 Gwei
    let gas_price = U256::from(2_140_000_000);
    let tx = TransactionRequest::new()
        .to(address_to.clone())
        .value(U256::from(utils::parse_ether(0)?))
        .from(address_from.clone())
        .data(data)
        .gas_price(gas_price);

    let tx = client.send_transaction(tx, None).await?.await?;
    let json_tx = serde_json::json!(tx);
    let txid = json_tx["transactionHash"].to_string();

    println!("\nWeaveVM Data Settling TXID: {}\n", txid);
    Ok(txid)
}

pub async fn send_wvm_calldata(data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let provider: Provider<Http> = Provider::<Http>::try_from("https://testnet-rpc.wvm.dev")
        .expect("could not instantiate HTTP Provider");
    let private_key = get_env_var("wvm_data_settler").unwrap();
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(9496_u64);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    let address_from = WVM_DATA_SETTLER.parse::<Address>()?;
    let address_to = Address::zero();
    // send calldata tx to WeaveVM
    let txid = propagate_wvm_transaction(&client, &address_from, &address_to, data).await?;

    Ok(txid)
}
