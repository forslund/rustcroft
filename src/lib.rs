mod adapt_interface;
pub mod mycroft_message;
pub mod skill;
pub mod config;
pub mod dialog;

pub use mycroft_message::MycroftMessage;
pub use adapt_interface::{AdaptKeyword, AdaptIntent};
pub use skill::{MsgHandler, EventHandler, start_skill};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


