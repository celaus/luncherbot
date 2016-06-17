use venue::Venue;

pub trait LocationProvider {
    fn venues_near(&self, lat: f32, lng: f32) -> Option<Vec<Venue>>;
}
