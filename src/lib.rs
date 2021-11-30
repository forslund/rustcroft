mod adapt_interface;
mod mycroft_message;
pub mod skill;

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


