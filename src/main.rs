
use paho_mqtt as mqtt;
use dotenv::dotenv;

use std::{env, process, io};
use std::io::prelude::*;


fn main() {
    
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| {
        println!("Host not set");
        process::exit(1);
    });

    let username = env::var("USERNAME").unwrap_or_else(|_| String::from(""));
    let password = env::var("PASSWORD").unwrap_or_else(|_| String::from(""));

    let topic = env::var("TOPIC").unwrap_or_else(|_| {
        println!("Topic not set");
        process::exit(1);
    });

    let cli = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        println!("Error creating the client: {}", err);
        process::exit(1);
    });

    let mut conn_builder = mqtt::ConnectOptionsBuilder::new_ws();

    if !username.is_empty() {
        conn_builder.user_name(username);
    }

    if !password.is_empty() {
        conn_builder.password(password);
    }

    let conn_opts = conn_builder.finalize();

    if let Err(err) = cli.connect(conn_opts).wait() {
        println!("Unable to connect: {}", err);
        process::exit(1);
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = mqtt::Message::new(&topic, line.unwrap(), 0);
        let tok = cli.publish(msg);
        if let Err(e) = tok.wait() {
            println!("Error sending message: {:?}", e);
        }
    }
}