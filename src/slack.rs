extern crate serde_json;
extern crate chrono;

use http::{Request, BackendError};
use chrono::*;

use std::str::from_utf8;
use std::str::FromStr;
use std::error::Error;
use std::io::Read;
use serde_json::Value;
use websocket::{Receiver, Sender, Client, Message};
use websocket::client::request::Url;
use std::collections::BTreeMap;


pub trait SlackAPIConsumer {
    fn on_hello(&self) -> Option<ChatMessage>;
    fn on_message(&self,
                  text: &str,
                  ts: &NaiveDateTime,
                  user: &str,
                  channel: &str,
                  subtype: &Option<&str>)
                  -> Option<ChatMessage>;
}


const SLACK_API_BASE: &'static str = "https://slack.com/api/";

#[derive(Debug)]
enum InternalError {
    with_message {
        msg: String,
    },
}

#[derive(Debug)]
pub enum SlackResult {
    Ok,
    NotOk,
}



pub struct SlackAPI<'a> {
    token: String,
    callbacks: Option<&'a SlackAPIConsumer>,
}

impl<'a> SlackAPI<'a> {
    pub fn new(token: String) -> SlackAPI<'a> {
        SlackAPI {
            token: token,
            callbacks: None,
        }
    }

    fn decode(&self, payload: &str) -> Result<Value, InternalError> {
        println!("decoding {}", &payload);
        return match serde_json::from_str(&payload) {
            Ok(v) => Ok(v),
            Err(e) => Err(InternalError::with_message { msg: e.description().to_owned() }),
        };
    }

    fn encode(&self, payload: &ChatMessage) -> Result<String, InternalError> {
        return match serde_json::to_string(&payload) {
            Ok(v) => Ok(v),
            Err(e) => Err(InternalError::with_message { msg: e.description().to_owned() }),
        };
    }

    fn api_method(&self, method: String) -> String {
        return format!("{}/{}", &SLACK_API_BASE, method);
    }

    fn api_start_channel(&self) -> Result<String, BackendError> {
        let rq = Request::new(self.api_method("rtm.start".to_owned()));
        let encoded_body = format!("token={}", self.token);
        return rq.post(&encoded_body);
    }

    fn api_set_active(&self) -> Result<String, BackendError> {
        let rq = Request::new(self.api_method("users.setPresence".to_owned()));
        let encoded_body = format!("token={}&presence=active", self.token);
        return rq.post(&encoded_body);
    }

    fn handle_message(&self, event: &BTreeMap<String, Value>) -> Option<ChatMessage> {
        if let Some(f) = self.callbacks {

            let subtype = match event.get("subtype") {
                Some(v) => v.as_string(),
                _ => None,
            };

            let channel = event.get("channel").unwrap().as_string().unwrap();
            println!("<--------");
            let msg = match subtype.unwrap_or("") {
                "message_changed" => event.get("message").unwrap().as_object().unwrap(),
                "" => event,
                _ => event,
            };
            println!("<--------");
            let txt = msg.get("text").unwrap().as_string().unwrap();
            println!("<--------");
            let ts: f64 = msg.get("ts").unwrap().as_string().unwrap().parse().unwrap();
            let timestamp = NaiveDateTime::from_timestamp(ts as i64, 0);
            println!("<--------");
            return match msg.get("user") {
                Some(v) => f.on_message(txt, &timestamp, v.as_string().unwrap(), channel, &subtype),
                _ => None,
            };

        } else {
            return None;
        }
    }

    fn handle_event(&self, r: Value) -> Result<Option<ChatMessage>, InternalError> {
        println!("handling event {:?}", &r);
        if let Some(event) = r.as_object() {
            println!("#----");
            if let Some(event_type) = event.get("type") {
                println!("#----");
                if let Some(f) = self.callbacks {
                    println!("#----");
                    let s = match event_type.as_string().unwrap() {
                        "hello" => f.on_hello(),
                        "message" => self.handle_message(event),
                        _ => None,
                    };
                    return Ok(s);
                }
                return Ok(None);
            } else {
                return Err(InternalError::with_message { msg: "Not an event".to_owned() });
            }
        } else {
            return Err(InternalError::with_message { msg: "Not an object".to_owned() });
        }
    }

    pub fn set_callbacks(&mut self, callbacks: &'a SlackAPIConsumer) {
        self.callbacks = Some(callbacks);
    }

    /// Connect to the Slack RTM API
    pub fn connect(&self) -> Result<SlackResult, SlackResult> {
        match self.api_start_channel() {
            Ok(raw) => {
                if self.api_set_active().is_err() {
                    return Err(SlackResult::NotOk);
                }

                let response: RTMStartResponse = serde_json::from_str(&raw).unwrap();

                let url = Url::parse(&*response.url).unwrap(); // Get the URL
                let request = Client::connect(url).unwrap(); // Connect to the server
                let response = request.send().unwrap(); // Send the request
                response.validate().unwrap(); // Ensure the response is valid

                let client = response.begin(); // Get a Client
                let (mut sender, mut receiver) = client.split();
                for received in receiver.incoming_messages() {
                    println!("msg cool? {}", received.is_ok());
                    if received.is_ok() {
                        let ws_msg: Message = received.unwrap();
                        let payload_u8 = ws_msg.payload.into_owned();
                        let payload = from_utf8(&payload_u8).unwrap_or("");
                        println!("Received: {}", payload);
                        let response = self.decode(&payload)
                            .and_then(|result| self.handle_event(result));
                        println!("()=====");
                        if let Ok(msg) = response {
                            println!("trying to send response");
                            if let Some(txt) = msg {
                                let response_text = self.encode(&txt).unwrap();
                                println!("{}", &response_text);
                                let message = Message::text(&*response_text);
                                let sent = sender.send_message(&message);
                            }
                        }
                    } else {
                        let ws_err = received.err();
                        println!("Error {:?}", ws_err)
                    }
                }
                return Ok(SlackResult::Ok);
            }
            _ => return Err(SlackResult::NotOk),
        }


    }
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct ChatMessage {
    pub id: i32,
    #[serde(rename="type")]
    pub t: String,
    pub channel: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct RTMStartRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    HelloEvent {
        t: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct RTMStartResponse {
    pub ok: bool,
    pub url: String,
}
