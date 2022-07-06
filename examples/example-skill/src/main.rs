use std::collections::HashMap;

use tokio_tungstenite::tungstenite::protocol::Message;
use futures_channel::mpsc::UnboundedSender;

use rustcroft::MycroftMessage;

use rustcroft::{AdaptIntent, AdaptKeyword};
use rustcroft::skill::{Skill, Speak, start_skill};
use rustcroft::dialog::{DialogCollection, DialogData};


///Print all utterances spoken by Mycroft
fn speak_handler(message: serde_json::Value, _: &UnboundedSender<Message>) {
    println!("Spoke {}", message["data"]["utterance"]);
}

///Speak an utterance
fn speak(bus_tx: &UnboundedSender<Message>, utterance: &str) {
    let speak_msg = MycroftMessage::new("speak")
        .with_data(serde_json::to_value(Speak{utterance: utterance.to_string()}).unwrap());
    bus_tx.unbounded_send(speak_msg.to_message()).unwrap();
}


///Handler for when the user says hello to rust
fn greet_intent_handler(_: serde_json::Value,
                        bus_tx: &UnboundedSender<Message>) {
    // TODO: A macro for the DialogData creation is needed and a speak_dialog
    //       helper would be nice.
    let dialogs = DialogCollection::from("dialog/en-us");
    let mut dialog_data: DialogData = HashMap::new();

    dialog_data.insert("thing".to_string(), "rust".to_string());
    speak(bus_tx, dialogs.get("hello", &dialog_data).unwrap().as_str())
}

///Handler for when the user says good bye to rust
fn goodbye_intent_handler(_: serde_json::Value,
                          bus_tx: &UnboundedSender<Message>) {
    speak(bus_tx, "Rust will never go away!")
}


#[tokio::main]
async fn main() {
    let mut skill_setup = Skill::new();
    skill_setup.handlers.add("speak", speak_handler);

    // Setup keywords
    let greet_keyword = AdaptKeyword {
            entity_value: "hello".to_string(),
            entity_type: "hello".to_string()
    };
    let rust_keyword = AdaptKeyword {
            entity_value: "rust".to_string(),
            entity_type: "rust".to_string()
    };
    let goodbye_keyword = AdaptKeyword {
            entity_value: "bye".to_string(),
            entity_type: "bye".to_string()
    };
    skill_setup.keywords.push(goodbye_keyword);
    skill_setup.keywords.push(greet_keyword);
    skill_setup.keywords.push(rust_keyword);

    // Setup intents
    let intent = AdaptIntent::new("greet")
                    .requiring("hello")
                    .requiring("rust");

    let intent2 = AdaptIntent::new("goodbye")
                    .requiring("bye")
                    .requiring("rust");

    skill_setup.intents.push((intent, greet_intent_handler));
    skill_setup.intents.push((intent2, goodbye_intent_handler));

    // Connect the skill to the message bus and register the intents and
    // their handlers
    start_skill(skill_setup).await;
}
