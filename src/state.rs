use nostr_sdk::prelude::*;
use std::time::Duration;
use tokio::sync::OnceCell;

pub static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn get_client() -> &'static Client {
    CLIENT
        .get_or_init(|| async {
            // Setup database
            let lmdb = NostrLMDB::open("./nostr").expect("Database is NOT initialized");

            // Client options
            let opts = Options::new()
                .gossip(true)
                .max_avg_latency(Duration::from_secs(2));

            // Setup Nostr Client
            let client = ClientBuilder::default().database(lmdb).opts(opts).build();

            // Add some bootstrap relays
            let _ = client.add_relay("wss://relay.damus.io").await;
            let _ = client.add_relay("wss://relay.primal.net").await;

            // Connect to all relays
            client.connect().await;

            // Return client
            client
        })
        .await
}
