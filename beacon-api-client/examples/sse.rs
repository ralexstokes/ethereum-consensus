use beacon_api_client::{mainnet::Client, Error, PayloadAttributesTopic};
use ethereum_consensus::Fork;
use futures_util::StreamExt;
use tracing::warn;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let client = Client::new(Url::parse("http://localhost:5052").unwrap());
    let mut events = client.get_events::<PayloadAttributesTopic>().await?;

    while let Some(event) = events.next().await {
        match event {
            Ok(event) => match event.version {
                Fork::Capella => {
                    let payload_attributes = event.data;
                    dbg!(payload_attributes);
                }
                _ => todo!(),
            },
            Err(err) => warn!(%err),
        }
    }
    Ok(())
}
