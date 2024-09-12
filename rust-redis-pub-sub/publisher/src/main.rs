use std::io::{stdin, stdout, Write};

use redis::{Commands, RedisError};

#[tokio::main]
async fn main() -> Result<(), RedisError> {
    let client = redis::Client::open("redis://127.0.0.1").unwrap();
    let mut pub_con = client.get_connection().unwrap();

    let channel = String::from("Test");

    loop {
        let mut s = String::new();
        print!("Enter message: ");

        let _ = stdout().flush();
        stdin().read_line(&mut s).unwrap();
       
        if s == String::from("exit\n") {
            break;
        }

        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        pub_con.publish(&channel, &s)?;
    }

    Ok(())

}
