use redis::{Commands, RedisError};

#[tokio::main]
async fn main() -> Result<(), RedisError>{
    let client = redis::Client::open("redis://127.0.0.1").unwrap();

    let mut sub_con = client.get_connection().unwrap();
    let mut pub_sub = sub_con.as_pubsub();

    let channel = String::from("Test");

    pub_sub.subscribe(&channel)?;

    loop {

        let msg = pub_sub.get_message().unwrap();
        let payload:String = msg.get_payload().unwrap();
        let channel:String = msg.get_channel().unwrap();
                                               
        println!("Message received {} from {}", payload, channel);
    }
}
