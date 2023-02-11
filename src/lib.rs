#[cfg(feature = "serde_json")]
pub mod model;

const BASE_URL: &str = "https://www.vivino.com/api/explore?";
// const RECORDS_PER_PAGE: u16 = 25;

pub struct Payload {
    /// "country_codes[]": "br","fr","us","de"
    pub country_codes: Option<Vec<String>>,
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
    //language=fr?
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            country_codes: Some(vec![String::from("us"), String::from("en")]),
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
        }
    }
}

pub fn get_url(payload: &Payload) -> String {
    let mut url = String::with_capacity(1024);
    url.push_str(BASE_URL);
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
        url.push_str(if x { "asc" } else { "desc" });
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
    url.replace("?&", "?")
}

// TODO: /carts/, https://sdk.split.io/api/splitChanges?since=-1 ?, https://auth.split.io/api/v2/auth?users=2bbfaa13-d94f-47d5-b09e-c01123b4d79a ?
// https://www.vivino.com/api/vintages/164825313/highlights?language=fr
// https://www.vivino.com/api/merchants/21896/shipping_policies?country=fr&bottle_quantity=1&total_amount=13.2&language=fr
// https://api.forethought.ai/workflow/widget-config -> Chatbot
// https://www.vivino.com/api/wines/1161809/tastes?language=fr
// https://www.vivino.com/api/wines/1161809/reviews?per_page=4&year=2019&language=fr
// https://www.vivino.com/api/wines/1161809/latest_reviews?per_page=4&year=2019&language=fr
// https://www.vivino.com/api/wineries/1215/vintages?language=fr
// https://www.vivino.com/api/prices?vintage_ids%5B%5D=154192912&vintage_ids%5B%5D=1520424&vintage_ids%5B%5D=4220270&vintage_ids%5B%5D=3485377&vintage_ids%5B%5D=162958002&vintage_ids%5B%5D=157570821&vintage_ids%5B%5D=2197646&language=fr
// https://www.vivino.com/api/wines/1161809/checkout_prices?language=fr

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use ureq::Agent;

    const USER_AGENT: &str = "";

    #[test]
    fn test_call_explore() {
        let agent: Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(120))
            .timeout_write(Duration::from_secs(10))
            .user_agent(USER_AGENT)
            .build();
        let payload = Payload {
            country_codes: Some(vec![String::from("us"), String::from("en")]),
            ..Default::default()
        };
        match agent.get(&get_url(&payload)).call() {
            Ok(resp) => {
                assert!(resp.status() == 200, "{:?}", resp);
                println!("{:?}", resp.into_string());
            }
            Err(e) => panic!("{}", e),
        }
    }
}
