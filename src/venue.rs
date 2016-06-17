pub struct Venue {
    pub name: String,
    pub vicinity: String,
    pub link: Option<String>,
    pub rating: u8
}


impl Venue {
    pub fn new(name: String, vicinity: String, link: Option<String>, rating: u8) -> Venue {
        Venue {
            name: name,
            vicinity: vicinity,
            link: link,
            rating: rating
        }
    }
}
