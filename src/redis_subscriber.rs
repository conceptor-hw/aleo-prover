extern crate redis;
use crate::message;
use crate::message::ProverMessage;
use crate::message::PubSubMessage;
use crate::redis_publisher;
use redis::{ControlFlow, PubSubCommands};
use std::error::Error;
use bincode;
use std::time::Duration;
use tokio::{task,time::sleep};
use tracing::{debug, error, info, warn};

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let mut con = client.get_connection().unwrap();

        let _: () = con
            .subscribe(&[channel], |msg| {
                let from_channel = msg.get_channel_name();
                match from_channel {
                    // from go transport pool server ProverMessage
                    "binary_channel_schedule" => {
                        let paylaod = msg.get_payload_bytes();
                        let message_obj: ProverMessage = bincode::deserialize(paylaod).unwrap();
                        info!("subcribe message 11111111111....{}",from_channel);
                        // message_handler::handle(message_obj);
                    }
                    // from go controller message
                    "mgt_channel_schedule" => {
                        let received: String = msg.get_payload().unwrap();
                        let message_obj = serde_json::from_str::<PubSubMessage>(&received).unwrap();
                        info!("subcribe message 22222222....{}",from_channel);
                        // message_handler::handle(message_obj);
                    }
                    _ => info!("something may be wrong..."),
                }

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}

pub fn start() ->Result<(), Box<dyn std::error::Error>>{
    println!("subcribe channel...");
    // start subscribe for redis
    if let Err(error) = subscribe(String::from(message::SUB_BINARY_CHANNEL)) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!(
            "connected to queue subscribe {:?}",
            message::SUB_BINARY_CHANNEL
        );
    }

    if let Err(error) = subscribe(String::from(message::SUB_MGT_CHANNEL)) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!(
            "connected to queue subscribe {:?}",
            message::SUB_MGT_CHANNEL
        );
    }

    // std::thread::sleep(Duration::from_secs(1));
    // task::spawn(async  move  {
    //     let mut i = 0;
    //     while i <= 30 {
    //         redis_publisher::publish_normal_message(message::PubSubMessage::new(message::Order::new(
    //             "message from rust".to_string(),
    //             0,
    //             i,
    //         ))).unwrap();
    //         sleep(std::time::Duration::from_secs(1)).await;
    //         i = i + 1;
    //     }
    // });

    Ok(())
}



