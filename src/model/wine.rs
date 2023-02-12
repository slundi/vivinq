use serde::Deserialize;

use super::{region::Region, image::Image};

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Wine {
    pub id: u32,
    pub name: String,
    pub seo_name: String,
    pub type_id: u8,
    pub vintage_type: u8,
    pub is_natural: bool,
    pub region: Region,
    pub background_image: Option<Image>,
}