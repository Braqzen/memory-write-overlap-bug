use fuels::prelude::*;

abigen!(Contract(
    name = "MyContract",
    abi = "./out/debug/memory-write-overlap-abi.json"
));

#[tokio::test]
async fn run() {
    // Skip to bottom for error call
    let mut wallets = launch_custom_provider_and_get_wallets(WalletsConfig::default(), None, None)
        .await
        .unwrap();

    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/memory-write-overlap.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet.clone(), TxPolicies::default())
    .await
    .unwrap();

    let hint_helper = MyContract::new(id, wallet.clone());

    // Errors
    let _ = hint_helper.methods().test().call().await.unwrap();
}
