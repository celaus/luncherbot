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
use toml::Parser;

use std::str::FromStr;
use chrono::*;

mod api;
mod http;
mod slack;

use http::Request;
use slack::{SlackAPI, SlackAPIConsumer, APIConsumerError};

use api::GoogleApi;

pub struct NamedLocation {
    name: String,
    longitude: f32,
    latitude: f32
}

struct LuncherBot {
    slack_api: SlackAPI,
    google_api: GoogleApi,
    locations: Vec<NamedLocation>
}

impl LuncherBot {

    fn new(slack_api: SlackAPI, google_api: GoogleApi, locations: Vec<NamedLocation>) -> LuncherBot {
        LuncherBot {
            slack_api: slack_api,
            google_api: google_api,
            locations: locations,
        }
    }

    fn connect(&self) {
        self.slack_api.connect(self);

        //let message = Message::text("Hello, World!");
        //client.send_message(&message).unwrap(); // Send message
    }

    //fn run(&self) {
    //    println!("Listening on {}:{}", self.interface, self.port);
    //    let server = Server::bind((self.interface, self.port)).unwrap();
    //
    //    for connection in server {
    //        println!("Connection established!");
    //        thread::spawn(move || {
    //           let request = connection.unwrap().read_request().unwrap(); // Get the request
    //           println!("{:?}", &request.path());
    //           let response = request.accept(); // Form a response
    //           let mut client = response.send().unwrap(); // Send the response
    //
    //           let api = GoogleApi::new("AIzaSyC2-340OPxHTsOps7Lj-rSp78Mvj-6i27w".to_owned());
    //
    //           let places = api.nearby(52.501862, 13.411262, "restaurant".to_owned()).unwrap();
    //           let message = Message::text(serde_json::to_string(&places).unwrap_or("[]".to_owned()));
    //        //    let message = Message::text("abc");
    //           let _ = client.send_message(&message);
    //
    //           for msg in client.incoming_messages() {
    //               match msg {
    //                   Ok(m) => {
    //                       let message: Message = m;
    //                       println!("Recv: {:?}", message);
    //                   },
    //                   _ =>  break
    //               }
    //           }
    //       });
    //    }
    //}
}

impl SlackAPIConsumer for LuncherBot {
    fn on_hello(&self) -> Result<String, APIConsumerError> {
        println!("HELLO EVENT");
        return Ok("hello".to_owned());
    }

    fn on_message(&self, text: String, ts: DateTime<UTC>, user: String, channel: String, subtype: String) -> Result<String, APIConsumerError> {
        return Ok("hello".to_owned());
    }
}

fn main() {

    let mut buffer = String::new();
    let mut f = try!(File::open("foo.txt"));
    try!(f.read_to_string(&mut buffer));

    let locations = vec!(NamedLocation {
        name: "BLN".to_owned(),
        latitude: 52.501862,
        longitude: 13.411262
    });
    let value = toml::Parser::new(&buffer).parse().unwrap();
    let google_api_key = value.lookup("keys.google").unwrap().as_str().unwrap();
    let slack_api_key = value.lookup("keys.slack").unwrap().as_str().unwrap();


    let google = GoogleApi::new(google_api_key);
    let slack = SlackAPI::new(slack_api_key);
    let srv = LuncherBot::new(slack, google, locations);
    srv.connect();
}
