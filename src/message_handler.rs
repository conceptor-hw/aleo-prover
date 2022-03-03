use crate::message::RedisMessage;

pub fn handle(message: RedisMessage) {
    println!(
        "subscribe: id{} channel {} desciption {} index:{}",
        message.id, message.channel, message.payload.description, message.payload.index,
    );
}
