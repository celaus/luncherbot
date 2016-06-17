
extern crate serde_json;
extern crate hyper;

use locationprovider::LocationProvider;
use self::hyper::{Client, Url};
use std::io::Read;
use serde_json::Value;
use venue::Venue;
use std::u8;

const PLACES_RADARSEARCH: &'static str = "https://api.foursquare.com/v2/venues/explore";
const PLACES_SEARCH_RADIUS: u32 = 2000;

#[derive(Debug)]
pub struct ApiError {}

pub struct FsVenueApi {
    client_id: String,
    client_secret: String
}

impl FsVenueApi {
    const CATEGORIES: &'static str = "food";

    pub fn new(client_id: String, client_secret: String) -> FsVenueApi {
        FsVenueApi {
            client_id: client_id,
            client_secret: client_secret
        }
    }

    fn decode(&self, payload: String) -> Option<Value> {
        return serde_json::from_str(&payload).ok();
    }

    fn request(&self, loc: &String, t: &str) -> Result<String, ApiError> {
        let client = Client::new();

        let mut url: hyper::Url = Url::parse(&PLACES_RADARSEARCH).unwrap();
        url.query_pairs_mut().clear()
            .append_pair("client_id", &self.client_id)
            .append_pair("client_secret", &self.client_secret)
            .append_pair("section", t)
            .append_pair("openNow", "1")
            .append_pair("ll", &loc)
            .append_pair("v", "20160617")
            .append_pair("llAcc", "10")
            .append_pair("limit", "50")
            .append_pair("sortByDistance", "1");

        let mut res = client.get(url).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return Ok(body);
    }
}
impl LocationProvider for FsVenueApi {

    fn venues_near(&self, lat: f32, lng: f32) -> Option<Vec<Venue>> {
        let body = self.request(&format!("{},{}", lat, lng), FsVenueApi::CATEGORIES).unwrap();
        let parsed: Value = self.decode(body).unwrap();
        let groups = parsed.lookup("response.groups").unwrap();
        let _recommended = groups.as_array().unwrap().iter().nth(0).unwrap();
        let recommended = _recommended.as_object().unwrap();
        match recommended.get("items").unwrap().as_array() {
            Some(items) => {
                let mut venues = Vec::with_capacity(items.len());
                for _item in items {
                    if let Some(item) = _item.as_object() {
                        let v = item.get("venue").unwrap();
                        let v_obj = v.as_object().unwrap();
                        let name = v_obj.get("name").unwrap().as_string().unwrap().to_owned();
                        let link = match item.get("url") {
                            Some(s) => Some(s.as_string().unwrap().to_owned()),
                            _ => None
                        };
                        let _vicinity = v.lookup("location.formattedAddress").unwrap().as_array().unwrap();
                        let vicinity = _vicinity.into_iter()
                            .map(|x| x.as_string().unwrap())
                            .fold(String::new(), |x, y| x + " " + y);
                        venues.push(Venue::new(name, vicinity, link, u8::MAX));
                    }
                }
                return Some(venues);
            },
            _ => return Some(vec![])
        };
    }
}
