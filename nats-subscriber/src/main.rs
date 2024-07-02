use async_nats::jetstream;
use futures::StreamExt;
use std::error::Error;

const NATS_URL: &str = "nats://nats:4222";
const TOPIC: &str = "engine-sensor";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = async_nats::connect(NATS_URL).await?;
    let mut subscription = client.subscribe(TOPIC).await?;

    while let Some(message) = subscription.next().await {
        println!(
            "Received message: {:?}",
            String::from_utf8(message.payload.to_vec())
        );
    }

    Ok(())
}
