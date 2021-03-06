use ureq::Agent;
use std::time::Duration;

const BASE_URL: &str = "https://www.vivino.com/api/";
const RECORDS_PER_PAGE: u16 = 25;
const USER_AGENT: &str = "";

pub struct Payload<'a> {
    /// "country_codes[]": "br","fr","us","de"
    pub country_codes: Option<Vec<&'a str>>,
    /// "food_ids[]": 20,
    pub food_ids: Option<Vec<u16>>,
    /// "grape_ids[]": 3,
    pub grape_ids: Option<Vec<u16>>,
    /// "grape_filter": "varietal",
    pub grape_filter: Option<String>,
    /// "min_rating": 3.7, from 0.0 to 5.0
    pub min_rating: Option<f32>,
    /// "order_by": "ratings_average",
    pub order_by: Option<String>,
    /// "order": "desc", or asc (true)
    pub order: Option<bool>,
    /// "price_range_min": 25,
    pub price_range_min: Option<u32>,
    /// "price_range_max": 100,
    pub price_range_max: Option<u32>,
    /// "region_ids[]": 383,
    pub region_ids: Option<Vec<u32>>,
    /// "wine_style_ids[]": 98,
    pub wine_style_ids: Option<Vec<u16>>,
    /// "wine_type_ids[]": 1,
    /// "wine_type_ids[]": 2,
    /// "wine_type_ids[]": 3,
    /// "wine_type_ids[]": 4,
    /// "wine_type_ids[]": 7,
    /// "wine_type_ids[]": 24,
    pub wine_type_ids: Option<u16>,
}

impl Default for Payload<'_> {fn default() -> Self {Payload {
    country_codes: Some(vec!["us", "fr"]),
    food_ids: None,
    grape_ids: None,
    grape_filter: None,
    min_rating: Some(3.7),
    order_by: None,
    order: None,
    price_range_min: None,
    price_range_max: None,
    region_ids: None,
    wine_style_ids: None,
    wine_type_ids: None,
}}}

pub fn get(payload: &Payload) -> Result<ureq::Response, ureq::Error> {
    let mut url = String::from(BASE_URL);
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(120))
        .timeout_write(Duration::from_secs(10))
        .user_agent(USER_AGENT)
        .build();
    url.push_str("explore/explore?");
    if let Some(x) = &payload.country_codes {
        for y in x {
            url.push_str("&country_codes[]=");
            url.push_str(y);
        }
    }
    if let Some(x) = &payload.food_ids {
        for y in x {
            url.push_str("&food_ids[]=");
            url.push_str(&y.to_string());
        }
    }
    if let Some(x) = &payload.grape_ids {
        for y in x {
            url.push_str("&grape_ids[]=");
            url.push_str(&y.to_string());
        }
    }
    if let Some(x) = &payload.grape_filter {
        url.push_str("&grape_filter=");
        url.push_str(x);
    }
    if let Some(x) = &payload.min_rating {
        url.push_str("&min_rating=");
        url.push_str(&x.to_string());
    }
    if let Some(x) = &payload.order_by {
        url.push_str("&order_by=");
        url.push_str(x);
    }
    if let Some(x) = payload.order {
        url.push_str("&order=");
        url.push_str(if x {"asc"} else {"desc"});
    }
    if let Some(x) = payload.price_range_min {
        url.push_str("&price_range_min=");
        url.push_str(&x.to_string());
    }
    if let Some(x) = payload.price_range_max {
        url.push_str("&price_range_max=");
        url.push_str(&x.to_string());
    }
    if let Some(x) = &payload.region_ids {
        for y in x {
            url.push_str("&region_ids[]=");
            url.push_str(&y.to_string());
        }
    }
    if let Some(x) = &payload.wine_style_ids {
        for y in x {
            url.push_str("&wine_style_ids[]=");
            url.push_str(&y.to_string());
        }
        
    }
    agent.get(&url.replace("?&","?")).call()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pl = Payload{country_codes:Some(vec!["fr","us"]), ..Default::default()};
        let out = get(&pl);
        let res = out.unwrap();
        assert!(res.status() == 200, "{:?}", res);
        println!("{:?}", res.into_string());
    }
}
