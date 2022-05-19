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
