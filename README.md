# VivinQ: wine data library from [Vivino](https://www.vivino.com/) data

This project is based on the python project [Viviner](https://github.com/gugarosa/viviner/).

It prepare an URL to query the Vivinio API. You are free to use any HTTP client to perform requests.

## Usage

```rust
fn main() {
    //get all wines from France and United States
    let pl = vivinq::Payload{country_codes:Some(vec![String::from("fr"), String::from("us")]), min_rating:4.2, ..Default::default()};
    let url: String = vivinq::get_url(&pl);
    // perform your request
}
```

It seems, you don't need to specify a USER_AGENT in your HTTP request, you can send an empty string `User-Agent: `.

The Payload is defined in lib.rs and parameters are optionals:

```rust
pub struct Payload {
    /// "country_codes[]": "br","fr","us","de", ...
    pub country_codes: Option<Vec<String>,
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
    pub wine_type_ids: Option<u16>,
}
```

## Features

`serde` (enabled by default). It allows you to deserialize Vivino JSON using serde.
