# VivinQ: wine data library from [Vivino](https://www.vivino.com/) data

This project is based on the python project [Viviner](https://github.com/gugarosa/viviner/).

It uses the [ureq](https://github.com/algesten/ureq) blocking HTTP client. If you want asynchronous jobs you are free to choose your own.

## Usage

```rust
fn main() {
    //get all wines from France and United States
    let pl = vivinq::Payload{country_codes:Some(vec!["fr","us"]), min_rating:4.2, ..Default::default()};
    let resp = vivinq::get(&pl); //returns a Result<ureq::Response, ureq::Error>
}
```

The Payload is defined in lib.rs and parameters are optionals:

```rust
pub struct Payload<'a> {
    /// "country_codes[]": "br","fr","us","de", ...
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
    pub wine_type_ids: Option<u16>,
}
```
