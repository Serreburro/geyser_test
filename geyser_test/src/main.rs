#[tokio::main]
async fn main() {
    dotenv().ok();
    monitor_meteora_pools().await.unwrap();
}

use futures_util::{stream, StreamExt};
use solana_sdk::pubkey::Pubkey;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use dotenv::dotenv;
use tonic::transport::{Channel, ClientTlsConfig};
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::geyser;
use yellowstone_grpc_proto::geyser::subscribe_request_filter_accounts_filter::Filter::Memcmp;
use yellowstone_grpc_proto::geyser::subscribe_request_filter_accounts_filter_memcmp::Data::Base58;
use yellowstone_grpc_proto::geyser::{SubscribeRequest, SubscribeRequestFilterAccounts, SubscribeRequestFilterAccountsFilter, SubscribeRequestFilterAccountsFilterMemcmp, SubscribeUpdate};
use yellowstone_grpc_proto::tonic::{Request, Streaming};
use yellowstone_grpc_proto::tonic::codegen::http;

pub async fn monitor_meteora_pools() -> Result<(), Box<dyn Error>> {
    let mut subscription = HashMap::new();

    subscription.insert(
        "filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec!["LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"
                .parse()
                .unwrap()],
            filters: vec![SubscribeRequestFilterAccountsFilter {
                filter: Some(Memcmp(SubscribeRequestFilterAccountsFilterMemcmp {
                    offset: 0,
                    data: Some(Base58("GUunkrC2gRJ".to_string())),
                })),
            }],
            nonempty_txn_signature: None,
        },
    );

    let mut stream = get_stream(subscription).await?;

    while let Some(update) = stream.next().await {
        match update {
            Ok(response) => {
                if let Some(geyser::subscribe_update::UpdateOneof::Account(account)) =
                    response.update_oneof
                {
                    tokio::task::spawn_blocking(move || unsafe {
                        let pubkey = Pubkey::new_from_array(
                            account.clone().account.unwrap().pubkey[0..32]
                                .try_into()
                                .expect("slice with incorrect length"),
                        );

                        let slot = account.slot;
                        println!("{} {:?}", pubkey.to_string(), slot);
                    });
                }
            }
            Err(error) => return Err(error.to_string().into()),
        }
    }
    Ok(())
}

pub async fn get_stream(
    accounts_map: HashMap<String, SubscribeRequestFilterAccounts>,
) -> Result<Streaming<SubscribeUpdate>, Box<dyn Error>> {
    let request = SubscribeRequest {
        accounts: accounts_map,
        commitment: Option::from(0i32),
        ..Default::default()
    };
    let request_stream = stream::once(async { request });
    let request = Request::new(request_stream);
    let grpc_url = env::var("GRPC_URL").unwrap();
    let uri: http::Uri = grpc_url.parse().unwrap();
    let tls_config = ClientTlsConfig::new()
        .domain_name("va.o7node.com")
        .with_native_roots();
    let channel = Channel::builder(uri)
        .tls_config(tls_config)?
        .connect()
        .await?;
    let mut client = GeyserClient::new(channel);
    let stream = client.subscribe(request).await?.into_inner();
    Ok(stream)
}
