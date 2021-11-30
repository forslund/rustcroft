use tokio_tungstenite::tungstenite::protocol::Message;
use futures_channel::mpsc::UnboundedSender;

use rustcroft::MycroftMessage;

use rustcroft::{AdaptIntent, AdaptKeyword};
use rustcroft::skill::{Skill, Speak, start_skill};


fn speak_handler(message: serde_json::Value, _: &UnboundedSender<Message>) {
    println!("Spoke {}", message["data"]["utterance"]);
}

fn speak(bus_tx: &UnboundedSender<Message>, utterance: &str) {
    let speak_msg = MycroftMessage::new("speak")
        .with_data(serde_json::to_value(Speak{utterance: utterance.to_string()}).unwrap());
    bus_tx.unbounded_send(speak_msg.to_message()).unwrap();
}

fn test_intent_handler(_: serde_json::Value, bus_tx: &UnboundedSender<Message>) {
    speak(bus_tx, "hello from a rust handler")
}

fn test_intent_handler2(_: serde_json::Value, bus_tx: &UnboundedSender<Message>) {
    speak(bus_tx, "hello from another rust handler")
}


#[tokio::main]
async fn main() {
    let mut skill_setup = Skill::new();
    skill_setup.handlers.add("speak", speak_handler);

    let intent = AdaptIntent::new("test")
                    .requiring("rust");

    let intent2 = AdaptIntent::new("test2")
                    .requiring("rust2");

    let keyword = AdaptKeyword {
            entity_value: "rust".to_string(),
            entity_type: "rust".to_string()
    };
    let keyword2 = AdaptKeyword {
            entity_value: "rust2".to_string(),
            entity_type: "rust2".to_string()
    };
    skill_setup.keywords.push(keyword);
    skill_setup.keywords.push(keyword2);
    skill_setup.intents.push((intent, test_intent_handler));
    skill_setup.intents.push((intent2, test_intent_handler2));

    start_skill(skill_setup).await;
}
