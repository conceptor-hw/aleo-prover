extern crate redis;

use crate::message::ProverMessage;
use crate::message::PubSubMessage;
use redis::Commands;
use std::error::Error;
use bincode;

pub fn publish_message(channel: &str, msg: ProverMessage) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;
    // println!("publishing message to go channel {:?}", msg);
    let serial_data = bincode::serialize(&msg).unwrap();
    con.publish(channel, serial_data)?;

    Ok(())
}

pub fn publish_normal_message(msg: PubSubMessage) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    let json = serde_json::to_string(&msg)?;

    con.publish(msg.channel, json)?;

    Ok(())
}
