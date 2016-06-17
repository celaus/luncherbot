
extern crate serde_json;
extern crate hyper;

use self::hyper::{Client, Url};
use std::io::Read;

const PLACES_RADARSEARCH: &'static str = "https://maps.googleapis.com/maps/api/place/nearbysearch/json";
const PLACES_SEARCH_RADIUS: u32 = 2000;

#[derive(Serialize, Deserialize)]
pub struct Location {
    lat: f32,
    lng: f32
}

#[derive(Serialize, Deserialize)]
pub struct Geometry {
    location: Option<Location>
}

//{
//     "geometry" : {
//        "location" : {
//           "lat" : -33.867591,
//           "lng" : 151.201196
//        }
//     },
//     "icon" : "http://maps.gstatic.com/mapfiles/place_api/icons/travel_agent-71.png",
//     "id" : "a97f9fb468bcd26b68a23072a55af82d4b325e0d",
//     "name" : "Australian Cruise Group",
//     "opening_hours" : {
//        "open_now" : true
//     },
//     "photos" : [
//        {
//           "height" : 242,
//           "html_attributions" : [],
//           "photo_reference" : "CnRnAAAABjeoPQ7NUU3pDitV4Vs0BgP1FLhf_iCgStUZUr4ZuNqQnc5k43jbvjKC2hTGM8SrmdJYyOyxRO3D2yutoJwVC4Vp_dzckkjG35L6LfMm5sjrOr6uyOtr2PNCp1xQylx6vhdcpW8yZjBZCvVsjNajLBIQ-z4ttAMIc8EjEZV7LsoFgRoU6OrqxvKCnkJGb9F16W57iIV4LuM",
//           "width" : 200
//        }
//     ],
//     "place_id" : "ChIJrTLr-GyuEmsRBfy61i59si0",
//     "scope" : "GOOGLE",
//     "reference" : "CoQBeQAAAFvf12y8veSQMdIMmAXQmus1zqkgKQ-O2KEX0Kr47rIRTy6HNsyosVl0CjvEBulIu_cujrSOgICdcxNioFDHtAxXBhqeR-8xXtm52Bp0lVwnO3LzLFY3jeo8WrsyIwNE1kQlGuWA4xklpOknHJuRXSQJVheRlYijOHSgsBQ35mOcEhC5IpbpqCMe82yR136087wZGhSziPEbooYkHLn9e5njOTuBprcfVw",
//     "types" : [ "travel_agency", "restaurant", "food", "establishment" ],
//     "vicinity" : "32 The Promenade, King Street Wharf 5, Sydney"
//  }

#[derive(Serialize, Deserialize)]
pub struct GooglePlacesData {
    pub geometry: Geometry,
    pub icon: Option<String>,
    pub id: String,
    pub name: String,
    pub types: Vec<String>,
    pub vicinity: String,
    next_page_token: Option<String>
}

#[derive(Serialize, Deserialize)]
struct GoogleApiResponse {
    status: String,
    results: Vec<GooglePlacesData>
}

#[derive(Debug)]
pub struct ApiError {}

pub struct GoogleApi {
    api_key: String
}
enum ApiRequest {
    PaginationRequest{next_page_token: String},
    NewRequest{location: String, tpe: String}
}

impl GoogleApi {
    pub fn new(api_key: String) -> GoogleApi {
        GoogleApi {
            api_key: api_key
        }
    }

    fn request2(&self, loc: &String, t: &String) -> Result<String, ApiError> {

        return Ok("{
            \"status\": \"OK\",
            \"results\": [{
                 \"geometry\" : {
                    \"location\" : {
                       \"lat\" : -33.867591,
                       \"lng\" : 151.201196
                    }
                 },
                 \"icon\" : \"http://maps.gstatic.com/mapfiles/place_api/icons/travel_agent-71.png\",
                 \"id\" : \"a97f9fb468bcd26b68a23072a55af82d4b325e0d\",
                 \"name\" : \"Australian Cruise Group\",
                 \"opening_hours\" : {
                    \"open_now\" : true
                 },
                 \"photos\" : [
                    {
                       \"height\" : 242,
                       \"html_attributions\" : [],
                       \"photo_reference\" : \"CnRnAAAABjeoPQ7NUU3pDitV4Vs0BgP1FLhf_iCgStUZUr4ZuNqQnc5k43jbvjKC2hTGM8SrmdJYyOyxRO3D2yutoJwVC4Vp_dzckkjG35L6LfMm5sjrOr6uyOtr2PNCp1xQylx6vhdcpW8yZjBZCvVsjNajLBIQ-z4ttAMIc8EjEZV7LsoFgRoU6OrqxvKCnkJGb9F16W57iIV4LuM\",
                       \"width\" : 200
                    }
                 ],
                 \"place_id\" : \"ChIJrTLr-GyuEmsRBfy61i59si0\",
                 \"scope\" : \"GOOGLE\",
                 \"reference\" : \"CoQBeQAAAFvf12y8veSQMdIMmAXQmus1zqkgKQ-O2KEX0Kr47rIRTy6HNsyosVl0CjvEBulIu_cujrSOgICdcxNioFDHtAxXBhqeR-8xXtm52Bp0lVwnO3LzLFY3jeo8WrsyIwNE1kQlGuWA4xklpOknHJuRXSQJVheRlYijOHSgsBQ35mOcEhC5IpbpqCMe82yR136087wZGhSziPEbooYkHLn9e5njOTuBprcfVw\",
                 \"types\" : [ \"travel_agency\", \"restaurant\", \"food\", \"establishment\" ],
                 \"vicinity\" : \"32 The Promenade, King Street Wharf 5, Sydney\"
              }]}".to_owned());
    }

    fn request(&self, loc: &String, t: &String) -> Result<String, ApiError> {
        let client = Client::new();

        let mut url: hyper::Url = Url::parse(&PLACES_RADARSEARCH).unwrap();
        url.query_pairs_mut().clear()
            .append_pair("key", &self.api_key)
            .append_pair("location",&loc)
            //.append_pair("radius", &PLACES_SEARCH_RADIUS.to_string()) // only if not rankby=distance
            .append_pair("type", &t)
            .append_pair("rankby", "distance");
        //match rq {
        //    ApiRequest::PaginationRequest { next_page_token: token} => url.append_pair("pagetoken", token),
        //    ApiRequest::NewRequest {location: loc, tpe: t} => url
        //        .append_pair("location",&loc)
        //        //.append_pair("radius", &PLACES_SEARCH_RADIUS.to_string()) // only if not rankby=distance
        //        .append_pair("type", &t)
        //        .append_pair("rankby", "distance")
        //};
        let mut res = client.get(url).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return Ok(body);
    }

    pub fn nearby(&self, lat: f32, lon: f32, t: String) -> Result<Vec<GooglePlacesData>, ApiError> {
        let body = self.request(&format!("{},{}", lat, lon), &t).unwrap();
        let parsed: GoogleApiResponse = serde_json::from_str(&body).unwrap();

        return Ok(parsed.results);
    }
}
