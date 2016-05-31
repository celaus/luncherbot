extern crate serde_json;
extern crate chrono;

use http::Request;
use chrono::*;

use std::str::from_utf8;
use std::str::FromStr;
use std::io::Read;
use serde_json::Value;
use websocket::{Client, Message};
use websocket::client::request::Url;

pub struct APIConsumerError {

}
pub trait SlackAPIConsumer {
    fn on_hello(&self) -> Result<String, APIConsumerError>;
    fn on_message(&self, text: &String, ts: &DateTime<UTC>, user: &String, channel: &String, subtype: &String) -> Result<String, APIConsumerError>;
}


const SLACK_API_BASE: &'static str = "https://slack.com/api/";

pub enum SlackResult {
    Ok,
    NotOk
}

pub struct SlackAPI {
    pub token: String
}

impl SlackAPI {
    pub fn new(token: String) -> SlackAPI {
        SlackAPI {
            token: token
        }
    }

    fn api_method(&self, method: String) -> String {
        return format!("{}/{}", &SLACK_API_BASE, method);
    }

    /// Connect to the Slack RTM API
    pub fn connect<T>(&self, callbacks: &T) -> Result<SlackResult, SlackResult>
    where T: SlackAPIConsumer {
        let rq = Request::new(self.api_method("rtm.start".to_owned()));
        let encoded_body = format!("token={}", self.token);
        match rq.post(&encoded_body) {
            Ok(raw) => {
                let response: RTMStartResponse = serde_json::from_str(&raw).unwrap();

                let url = Url::parse(&*response.url).unwrap(); // Get the URL
                let request = Client::connect(url).unwrap(); // Connect to the server
                let response = request.send().unwrap(); // Send the request
                response.validate().unwrap(); // Ensure the response is valid

                let mut client = response.begin(); // Get a Client
                for msg in client.incoming_messages() {
                    let ws_msg: Message = msg.unwrap();
                    let payload_u8 = ws_msg.payload.into_owned();
                    let payload = from_utf8(&payload_u8).unwrap();
                    println!("{}", payload);

                    let r:Value = serde_json::from_str(&payload).unwrap();
                    let event = r.as_object().unwrap();
                    let event_type = event.get("type").unwrap();
                    let x =match event_type.as_string().unwrap() {
                        "hello" => callbacks.on_hello(),
                        "message" => {
                            let txt = event.get("text").unwrap().as_string().unwrap();
                            let subtype = event.get("subtype").unwrap().as_string().unwrap();
                            let channel = event.get("channel").unwrap().as_string().unwrap();
                            let user = event.get("user").unwrap().as_string().unwrap();
                            callbacks.on_message(txt, DateTime::now(), user, channel, subtype)
                            },
                        _ => Err(APIConsumerError{})
                    };
                }
                return Ok(SlackResult::Ok);
            },
            _ => return Err(SlackResult::NotOk)
        }


    }
}

#[derive(Serialize)]
pub struct RTMStartRequest{
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    HelloEvent {t: String},
}

#[derive(Serialize, Deserialize)]
pub struct RTMStartResponse{
    pub ok: bool,
    pub url: String,
//    {
//    "ok": true,
//    "url": "wss:\/\/ms9.slack-msgs.com\/websocket\/7I5yBpcvk",
//
//    "self": {
//        "id": "U023BECGF",
//        "name": "bobby",
//        "prefs": {
//            …
//        },
//        "created": 1402463766,
//        "manual_presence": "active"
//    },
//    "team": {
//        "id": "T024BE7LD",
//        "name": "Example Team",
//        "email_domain": "",
//        "domain": "example",
//        "icon": {
//            …
//        },
//        "msg_edit_window_mins": -1,
//        "over_storage_limit": false
//        "prefs": {
//            …
//        },
//        "plan": "std"
//    },
//    "users": [ … ],
//
//    "channels": [ … ],
//    "groups": [ … ],
//    "mpims": [ … ],
//    "ims": [ … ],
//
//    "bots": [ … ],
//}
}
