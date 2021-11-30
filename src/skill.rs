use tokio_tungstenite::{connect_async};
use url::Url;

use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::protocol::Message;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use futures_util::{future, pin_mut, StreamExt};

use super::MycroftMessage;

use super::{AdaptIntent, AdaptKeyword};

#[derive(Serialize, Deserialize)]
pub struct Speak {
    pub utterance: String
}


pub type MsgHandler = fn(serde_json::Value, &UnboundedSender<Message>);

pub struct EventHandler {
    handlers: HashMap<String, MsgHandler>
}


impl EventHandler {
    #[allow(dead_code)]
    pub fn add(&mut self, name: &str, func: MsgHandler) {
        let key = format!("\"{}\"", name);
        self.handlers.insert(key, func);
    }

    #[allow(dead_code)]
    pub fn call(&self, name: &String, msg: serde_json::Value, bus_tx: &UnboundedSender<Message>) {
        match self.handlers.get(name) {
            Some(handler) => handler(msg, &bus_tx),
            None => ()
        }
    }

    #[allow(dead_code)]
    pub fn add_adapt_handler(&mut self, 
                         bus_tx: &UnboundedSender<Message>,
                         intent: &AdaptIntent,
                         func: MsgHandler) {
        self.add(intent.name.as_str(), func);

        let intent_message = MycroftMessage::new("register_intent")
            .with_data(serde_json::to_value(intent).unwrap());
        bus_tx.unbounded_send(intent_message.to_message()).unwrap();
    }

    pub async fn handle_msg(&self, bus_tx: &UnboundedSender<Message>,
                        data: Vec<u8>) -> serde_json::Result<()> {
        let s = std::str::from_utf8(&data).unwrap();
        let msg: Value = serde_json::from_str(&s)?;
        self.call(&msg["type"].to_string(), msg, &bus_tx);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn new() -> EventHandler {
        EventHandler{ handlers: HashMap::new() }
    }

}


pub struct Skill {
   pub intents: Vec<(AdaptIntent, MsgHandler)>,
   pub keywords: Vec::<AdaptKeyword>,
   pub handlers: EventHandler
}

impl Skill {
    #[allow(dead_code)]
    pub fn new() -> Skill {
        Skill {
            handlers: EventHandler::new(),
            intents: Vec::<(AdaptIntent, MsgHandler)>::new(),
            keywords: Vec::<AdaptKeyword>::new()
        }
    }
}


pub async fn start_skill(mut skill_setup: Skill) {
    let (bus_tx, bus_rx) = futures_channel::mpsc::unbounded();

    let url = Url::parse("ws://localhost:8181/core").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();
    let write_to_ws = bus_rx.map(Ok).forward(write);

    
    for (intent, handler) in skill_setup.intents.iter() {
        skill_setup.handlers.add_adapt_handler(&bus_tx,
                                               intent.clone(),
                                               *handler);
    }
    
    let handle_message = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            skill_setup.handlers.handle_msg(&bus_tx, data).await.unwrap();
        })
    };

    for keyword in skill_setup.keywords.iter() {
        let keyword_message = MycroftMessage::new("register_vocab")
            .with_data(serde_json::to_value(keyword).unwrap());
        bus_tx.unbounded_send(keyword_message.to_message()).unwrap();
    }

    pin_mut!(write_to_ws, handle_message);
    future::select(write_to_ws, handle_message).await;
}
