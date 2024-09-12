use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions,
};

fn main() -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Press ctrl + C to exit");
    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("Received message number {} with body {}", i, body);
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended {:?}", other);
                break;
            }
        }
    }
    connection.close()
}
