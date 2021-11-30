use serde_json::{Value};
use tokio_tungstenite::tungstenite::protocol::Message;

pub struct MycroftMessage {
	msg_type: String,
	data: Value,
	context: Value
}

impl MycroftMessage {
    #[allow(dead_code)]
    pub fn new(msg_type: &str) -> MycroftMessage {
        MycroftMessage {
            msg_type: msg_type.to_string(),
            data: serde_json::json!({}),
            context: serde_json::json!({}),
        }
    }

    #[allow(dead_code)]
    pub fn with_data(mut self, data_obj: Value) -> MycroftMessage{
        self.data = data_obj;
        self
    }
    
    #[allow(dead_code)]
    pub fn to_string(self) -> String {
        format!("{{\"type\":\"{}\",\"data\":{},\"context\":{}}}",
                self.msg_type,
                self.data.to_string(),
                self.context.to_string()
        )
    }

    #[allow(dead_code)]
    pub fn to_message(self) -> Message {
        let string_repr = self.to_string();
        Message::text(string_repr)
    }
}
