use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Grape {
    id: u32,
    name: String,
    seo_name: String,
    has_detailed_info: bool,
    wineries_count: u32,
}