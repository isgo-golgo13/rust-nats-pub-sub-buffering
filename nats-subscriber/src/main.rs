// nats-subscriber/src/main.rs

use async_nats::jetstream::context::Context;
use async_nats::jetstream::stream::Stream;
use async_nats::Client;
use futures::StreamExt;
use std::error::Error;

const NATS_URL: &str = "nats://nats:4222";
const TOPIC: &str = "engine-sensor";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = async_nats::connect(NATS_URL).await?;
    let context = Context::new(client);

    let mut subscription = context.subscribe(TOPIC.to_string()).await?;

    while let Some(message) = subscription.next().await {
        println!("Received message: {:?}", String::from_utf8(message.data));
    }

    Ok(())
}
