use async_nats::jetstream;
use bytes::Bytes;
use std::error::Error;
use tokio::time::{sleep, Duration};

const NATS_URL: &str = "nats://nats:4222";
const TOPIC: &str = "engine-sensor";
const MESSAGE: &str = "000000000000001";
const BATCH_SIZE: usize = 25;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = async_nats::connect(NATS_URL).await?;
    let jetstream = async_nats::jetstream::new(client);

    loop {
        let mut messages: Vec<bytes::Bytes> = vec![];

        for _ in 0..BATCH_SIZE {
            messages.push(MESSAGE.into());
        }

        for message in messages {
            jetstream.publish(TOPIC.to_string(), message.into()).await?;
        }

        println!("Published a batch of {} messages", BATCH_SIZE);

        sleep(Duration::from_secs(1)).await; // Pause for a second before the next batch
    }
}
