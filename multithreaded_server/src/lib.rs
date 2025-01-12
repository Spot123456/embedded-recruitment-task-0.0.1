pub mod client;
pub mod server;

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}