use toncenter::client::{ApiClientV3, ApiKey, Network};
#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = "a8b61ced4be11488cb6e82d65b93e3d4a29d20af406aed9688b9e0077e2dc742".to_string();
    let address = "0QA6W2spRJ6D-AUf6PHTfKJCib63ZJU6fK8BxHVp322UlXe4";

    let api_client = ApiClientV3::new(Network::Testnet, Some(ApiKey::Header(api_key)));

    /*
    match api_client.get_address_information(address).await {
        Ok(info) => println!("Address info: {:#?}", info),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
     */

    /*
    match api_client.run_get_method(address, "seqno", &[]).await {
        Ok(info) => {
            if info.exit_code == 0 {
                let stack_type_value = info.stack.first().unwrap();
                let seqno: u64 =
                    u64::from_str_radix(stack_type_value.value.trim_start_matches("0x"), 16)
                        .unwrap();
                println!("{}", seqno);
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
     */

    /*
    let boc = "";
    match api_client.send_message(boc).await {
        Ok(info) => println!("message hash: {:#?}", info.message_hash),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
     */

    let owner_address = address;
    let jetton_address = "kQC6P2OM8eudgyuPN6LHmAzL5B8s3-xtybV-FHlEx4_yhKBN"; // jetton master
    match api_client
        .get_jetton_wallets(owner_address, jetton_address)
        .await
    {
        Ok(info) => {
            let wallet = info.jetton_wallets.first().unwrap();
            // todo: show balance with jetton contract precision
            println!("wallet balance: {:#?}", wallet.balance);
        }

        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}
