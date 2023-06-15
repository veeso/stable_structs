use argh::FromArgs;
use candid::{Decode, Encode, Principal};
use did::Transaction;
use std::path::PathBuf;

mod agent;

#[derive(FromArgs)]
#[argh(description = "")]
struct Args {
    #[argh(option, description = "sender address")]
    pub from: u8,
    #[argh(option, description = "recipient address")]
    pub to: u8,
    #[argh(option, description = "value to send")]
    pub value: u8,
    #[argh(
        option,
        description = "network url",
        default = r#""http://localhost:4943".to_string()"#
    )]
    pub network: String,
    #[argh(option, description = "identity path")]
    pub identity: PathBuf,
    #[argh(option, description = "canister id")]
    pub canister_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    let principal = Principal::from_text(&args.canister_id).expect("invalid canister id");

    let agent = agent::init_agent(&args.identity, &args.network).await?;
    println!("agent initialized; sending transaction");

    let arg = Encode!(&args.from, &args.to, &args.value).expect("failed to encode args");

    let res = agent
        .update(&principal, "add_tx")
        .with_arg(arg)
        .call_and_wait()
        .await?;

    let key = Decode!(&res, u64).unwrap();
    println!("inserted transaction {key}");

    // check transaction
    let arg = Encode!(&key).expect("failed to encode args");

    let res = agent
        .query(&principal, "get_tx")
        .with_arg(arg)
        .call()
        .await?;

    let transaction = Decode!(&res, Option<Transaction>).unwrap();
    assert!(transaction.is_some());

    println!("transaction inserted {:?}", transaction);

    let arg = Encode!(&()).expect("failewd to encode args");
    // get latest transaction key
    let res = agent
        .query(&principal, "get_latest_key")
        .with_arg(arg)
        .call()
        .await?;

    let key = Decode!(&res, u64).unwrap();
    println!("latest tx key: {key}");

    Ok(())
}
