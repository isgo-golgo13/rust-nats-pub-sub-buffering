// nats-publisher/src/main.rs

use async_nats::jetstream::context::Context;
use async_nats::{Client, Message};
use futures::stream::StreamExt;
use std::error::Error;
use tokio::time::{sleep, Duration};

const NATS_URL: &str = "nats://nats:4222";
const TOPIC: &str = "engine-sensor";
const MESSAGE: &str = "000000000000001";
const BATCH_SIZE: usize = 25;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = async_nats::connect(NATS_URL).await?;
    let context = Context::new(client);

    let mut messages = vec![];

    loop {
        for _ in 0..BATCH_SIZE {
            messages.push(Message {
                subject: TOPIC.into(),
                data: MESSAGE.into(),
                ..Default::default()
            });
        }

        context.publish_all(&messages).await?;
        messages.clear();

        println!("Published a batch of {} messages", BATCH_SIZE);

        sleep(Duration::from_secs(1)).await; // Pause for a second before the next batch
    }
}
