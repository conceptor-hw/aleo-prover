use crate::message::ProverMessage;


pub fn handle_pubsub_messsage(message: ProverMessage) {
    println!("subscribe handle is : id{:?} ", message);
}