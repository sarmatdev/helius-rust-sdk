use helius::config::Config;
use helius::error::HeliusError;
use helius::rpc_client::RpcClient;
use helius::types::{Asset, Cluster, GetAssetBatch, GetAssetOptions};
use helius::Helius;

use reqwest::Client;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), HeliusError> {
    let api_key: &str = "your_api_key";
    let cluster: Cluster = Cluster::MainnetBeta;

    let config: Arc<Config> = Arc::new(Config::new(api_key, cluster)?);
    let client: Client = Client::new();
    let rpc_client: Arc<RpcClient> = Arc::new(RpcClient::new(Arc::new(client.clone()), Arc::clone(&config)).unwrap());

    let helius: Helius = Helius {
        config,
        client,
        rpc_client,
    };

    let request: GetAssetBatch = GetAssetBatch {
        ids: vec![
            "81bxPqYCE8j34nQm7Rooqi8Vt3iMHLzgZJ71rUVbQQuz".to_string(),
            "CWHuz6GPjWYdwt7rTfRHKaorMwZP58Spyd7aqGK7xFbn".to_string(),
        ],
        display_options: Some(GetAssetOptions {
            show_collection_metadata: true,
            ..Default::default()
        }),
    };

    let response: Result<Vec<Option<Asset>>, HeliusError> = helius.rpc().get_asset_batch(request).await;
    println!("Assets: {:?}", response);

    Ok(())
}
