use serde::Deserialize;

use super::country::Country;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Region {
    id: u32,
    name: String,
    seo_name: String,
    country: Country,
    parent_id: u32,
    background_image: Option<String>,
    //class: Class,
    statistics: RegionStatistics,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct RegionStatistics {
    wineries_count: u32,
    wines_count: u32,
    sub_regions_count: u16,
    parent_regions_count: u16,
}

