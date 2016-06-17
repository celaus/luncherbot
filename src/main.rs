#![feature(custom_derive, plugin, associated_consts)]
#![plugin(serde_macros)]

extern crate serde_json;
extern crate websocket;
extern crate chrono;
extern crate toml;
extern crate rand;

mod google;
mod http;
mod slack;
mod luncherbot;
mod venue;
mod locationprovider;
mod foursquare;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use toml::Parser;
use slack::{SlackAPI, SlackAPIConsumer};
use luncherbot::{LuncherBot, NamedLocation};
use google::GoPlacesApi;
use foursquare::FsVenueApi;
use locationprovider::LocationProvider;


fn read_api_keys(filename: &str) -> Result<(String, String, String, String), io::Error> {
    let mut buffer = String::new();
    let mut f = try!(File::open(filename));
    try!(f.read_to_string(&mut buffer));

    let value = Parser::new(&buffer).parse().unwrap();
    let keys = value.get("keys").unwrap();
    let google_api_key = keys.lookup("google").unwrap().as_str().unwrap().to_owned();
    let slack_api_key = keys.lookup("slack").unwrap().as_str().unwrap().to_owned();
    let fs_client_id = keys.lookup("fs_client_id").unwrap().as_str().unwrap().to_owned();
    let fs_client_secret = keys.lookup("fs_client_secret").unwrap().as_str().unwrap().to_owned();

    return Ok((google_api_key, slack_api_key, fs_client_id, fs_client_secret));
}

fn main() {
    let locations = vec!(NamedLocation {
        name: "BER".to_owned(),
        latitude: 52.501862,
        longitude: 13.411262
    },
    NamedLocation {
        name: "DBN".to_owned(),
        latitude: 47.405018,
        longitude: 9.742586
    });

    let api_keys = read_api_keys("config.toml").unwrap();
    let google = GoPlacesApi::new(api_keys.0);
    let fs = FsVenueApi::new(api_keys.2, api_keys.3);
    let srv = LuncherBot::new(&fs as &LocationProvider, locations);
    let mut slack = SlackAPI::new(api_keys.1);

    slack.set_callbacks(&srv as &SlackAPIConsumer);
    let exit = slack.connect();
}
