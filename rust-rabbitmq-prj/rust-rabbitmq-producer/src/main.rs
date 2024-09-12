use std::io::{stdin, stdout, Write};

use amiquip::{
    Connection, Exchange, Publish
};

fn main() -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    
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
        exchange.publish(Publish::new(s.as_bytes(), "hello"))?;
    }
    
    
    connection.close()
}
