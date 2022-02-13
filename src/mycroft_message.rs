use serde_json::{Value};
use tokio_tungstenite::tungstenite::protocol::Message;


#[allow(dead_code)]
/// Create a response message type from the original
pub fn build_response_type<S: AsRef<str>>(original_msg_type: S) -> String {
    let str_ref = original_msg_type.as_ref();
    println!("{}.response", str_ref);
    format!("{}.response", str_ref)
}


pub struct MycroftMessage {
	msg_type: String,
	data: Value,
	context: Value
}

impl MycroftMessage {
    #[allow(dead_code)]
    /// Create a new Message for the mycroft bus
    pub fn new(msg_type: &str) -> MycroftMessage {
        MycroftMessage {
            msg_type: msg_type.to_string(),
            data: serde_json::json!({}),
            context: serde_json::json!({}),
        }
    }

    #[allow(dead_code)]
    /// Create a response Message based on this message.
    pub fn response(self) -> MycroftMessage {
        let response_type = build_response_type(self.msg_type);
        MycroftMessage {
            msg_type: response_type,
            data: serde_json::json!({}),
            context: self.context,
        }
    }

    #[allow(dead_code)]
    /// Set the messages data
    pub fn with_data(mut self, data_obj: Value) -> MycroftMessage{
        self.data = data_obj;
        self
    }
    
    #[allow(dead_code)]
    /// serialize to string
    pub fn to_string(self) -> String {
        format!("{{\"type\":\"{}\",\"data\":{},\"context\":{}}}",
                self.msg_type,
                self.data.to_string(),
                self.context.to_string()
        )
    }

    #[allow(dead_code)]
    /// Convert to tungstenite Message
    pub fn to_message(self) -> Message {
        let string_repr = self.to_string();
        Message::text(string_repr)
    }
}
