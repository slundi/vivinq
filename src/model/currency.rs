use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Currency {
    code: String,
    name: String,
    prefix: Option<String>,
    suffix: Option<String>,
}