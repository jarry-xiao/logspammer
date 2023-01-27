use ellipsis_client::EllipsisClient;
use solana_client::{
    rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig,
    transaction_executor::TransactionExecutor,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    feature_set::spl_associated_token_account_v1_1_0,
    instruction::Instruction,
    pubkey::Pubkey,
    rent::Rent,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    system_instruction, sysvar,
    transaction::{self, Transaction},
};
use std::env;
use std::str::FromStr;

pub fn get_payer_keypair() -> solana_sdk::signer::keypair::Keypair {
    match env::var("PAYER").is_ok() {
        true => Keypair::from_base58_string(&env::var("PAYER").expect("$PAYER is not set")[..]),
        false => read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).unwrap(),
    }
}

#[tokio::main]
async fn main() {
    let program_id = Pubkey::from_str("BB7RH5Dw3qSGPgQy4Xu3qmunJY9ShahCtMikpEPkqKrX").unwrap();

    let rpc =
        RpcClient::new_with_commitment("http://127.0.0.1:8899", CommitmentConfig::confirmed());

    let payer = get_payer_keypair();

    let ellipsis_client = EllipsisClient::from_rpc(rpc, &payer).unwrap();

    let ix = Instruction {
        program_id,
        data: vec![],
        accounts: vec![],
    };

    let new_acct = Keypair::new();
    let self_transfer_ix = system_instruction::create_account(
        &payer.pubkey(),
        &new_acct.pubkey(),
        100000,
        0,
        &program_id,
    );

    let tx = ellipsis_client
        .sign_send_instructions(vec![ix, self_transfer_ix.clone()], vec![&payer, &new_acct])
        .await
        .unwrap();

    let new_acct = Keypair::new();
    let self_transfer_ix = system_instruction::create_account(
        &payer.pubkey(),
        &new_acct.pubkey(),
        100000,
        0,
        &program_id,
    );
    let tx = ellipsis_client
        .sign_send_instructions(vec![self_transfer_ix], vec![&payer, &new_acct])
        .await
        .unwrap();
}
