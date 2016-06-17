extern crate serde_json;
extern crate websocket;
extern crate chrono;
extern crate toml;
extern crate rand;


use locationprovider::LocationProvider;
use rand::Rng;

use chrono::*;

use slack::{SlackAPIConsumer, ChatMessage};

pub struct NamedLocation {
    pub name: String,
    pub longitude: f32,
    pub latitude: f32
}

pub struct LuncherBot<'a>  {
    location_provider: &'a LocationProvider,
    locations: Vec<NamedLocation>
}

impl <'a>LuncherBot<'a> {
    const CATEGORIES: &'static str = "restaurant";
    const COMMAND: &'static str = "!luncherbot";
    const HELP: &'static str = "Hi! I'm luncherbot, your friendly lunch advisor.";
    const USAGE_PATTERN: &'static str = "Type: {} <location> in Slack to invoke me.";

    pub fn new(location_provider: &'a LocationProvider, locations: Vec<NamedLocation>) -> LuncherBot {
        LuncherBot {
            location_provider: location_provider,
            locations: locations,
        }
    }
}

impl  <'a>SlackAPIConsumer for LuncherBot<'a> {
    fn on_hello(&self) -> Option<ChatMessage> {
        return None;
    }

    fn on_message(&self, text: &str, ts: &NaiveDateTime, user: &str, channel: &str, subtype: &Option<&str>) -> Option<ChatMessage> {
        if let Some(mentioned_msg) = text.find(LuncherBot::COMMAND) {
            let txt = text.trim_left_matches(LuncherBot::COMMAND).trim().to_owned().to_uppercase();
            let places_found = match self.locations.iter().filter(|l| l.name == txt).next() {
                Some(location) => self.location_provider.venues_near(location.latitude, location.longitude),
                _ => None
            };

            let response_text = match places_found {
                Some(places) => {
                        println!("Available places: {}", places.iter()
                            .map(|p| &p.name)
                            .fold(String::new(), |p, c| p + "\n - "+ c));
                        let selection = rand::thread_rng().choose(&places).unwrap();
                        format!("Name: {}, address: {}", selection.name, selection.vicinity) },
                _ => format!("Nothing found for location '{}'", txt)
            };

            return Some(ChatMessage {
                id: 22,
                t: "message".to_owned(),
                channel: format!("{}", channel.to_owned()),
                text: response_text
            });
        }
        else {
            return None;
        }
    }
}
