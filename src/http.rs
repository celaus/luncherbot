extern crate hyper;

use self::hyper::{Client, Url};
use self::hyper::header::{Headers, ContentType};

use std::io::Read;

pub type BackendError = hyper::Error;

pub struct Request {
    url: String,
}

impl Request{
    pub fn new(url: String) -> Request {
        Request {
            url: url
        }
    }

    ///
    ///
    ///
    pub fn get(&self, params: Vec<(&String, &String)>) -> Result<String, BackendError> {
        let mut url: hyper::Url = Url::parse(&*self.url).unwrap();
        let client = Client::new();

        url.query_pairs_mut().clear();
        for p in params {
            url.query_pairs_mut().append_pair(p.0, p.1);
        }

        let mut res = client.get(url).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return Ok(body);
    }


    pub fn post(&self, body: &String) -> Result<String, BackendError> {
        let url: hyper::Url = Url::parse(&*self.url).unwrap();
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(ContentType::form_url_encoded());
        let mut res = client.post(url.as_str())
            .headers(headers)
            .body(body)
            .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return Ok(body);
    }

}
