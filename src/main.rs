#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde_json;
extern crate websocket;
extern crate chrono;
extern crate toml;


use websocket::{Client, Message};
use websocket::client::request::Url;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use toml::{Parser, Value};
use std::slice::SliceConcatExt;

use std::str::FromStr;
use chrono::*;

mod api;
mod http;
mod slack;

use http::Request;
use slack::{SlackAPI, SlackAPIConsumer, ChatMessage};

use api::GoogleApi;

pub struct NamedLocation {
    name: String,
    longitude: f32,
    latitude: f32
}

struct LuncherBot  {
    google_api: GoogleApi,
    locations: Vec<NamedLocation>
}

impl LuncherBot {

    fn new(google_api: GoogleApi, locations: Vec<NamedLocation>) -> LuncherBot {
        LuncherBot {
            google_api: google_api,
            locations: locations,
        }
    }
}

impl  SlackAPIConsumer for LuncherBot {
    fn on_hello(&self) -> Option<ChatMessage> {
        println!("HELLO EVENT");
        return None;
    }

    fn on_message(&self, text: &str, ts: &NaiveDateTime, user: &str, channel: &str, subtype: &Option<&str>) -> Option<ChatMessage> {
        let places = self.google_api.nearby(52.501862, 13.411262, "restaurant".to_owned()).unwrap();
        let x: Vec<&String> = places.iter().map(|p| &p.name).collect();
        println!("{:?}", x);
        return Some(ChatMessage {
            id: 22,
            t: "message".to_owned(),
            channel: format!("{}", channel.to_owned()),
            text: x.join(", ")
        })
    }
}

fn read_api_keys(filename: &str) -> Result<(String, String), io::Error> {
    let mut buffer = String::new();
    let mut f = try!(File::open(filename));
    try!(f.read_to_string(&mut buffer));

    let value = Parser::new(&buffer).parse().unwrap();
    let keys = value.get("keys").unwrap();
    let google_api_key = keys.lookup("google").unwrap().as_str().unwrap().to_owned();
    let slack_api_key = keys.lookup("slack").unwrap().as_str().unwrap().to_owned();
    return Ok((google_api_key, slack_api_key));
}

fn main() {
    let locations = vec!(NamedLocation {
        name: "BLN".to_owned(),
        latitude: 52.501862,
        longitude: 13.411262
    });

    let api_keys = read_api_keys("config.toml").unwrap();
    let google = GoogleApi::new(api_keys.0);
    let srv = LuncherBot::new(google, locations);
    let mut slack = SlackAPI::new(api_keys.1);
    slack.set_callbacks(&srv as &SlackAPIConsumer);
    slack.connect();
}
