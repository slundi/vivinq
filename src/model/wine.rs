use serde::Deserialize;

use super::{region::Region, image::Image};

/// "region": {
///     "id": 2448,
///     "name": "Côtes du Roussillon Villages",
///     "name_en": "",
///     "seo_name": "cotes-du-roussillon-villages",
///     "country": {
///         "code": "fr",
///         "name": "France",
///         "native_name": "France",
///         "seo_name": "france",
///         "currency": {
///             "code": "EUR",
///             "name": "Euros",
///             "prefix": "€",
///             "suffix": null
///         },
///         "regions_count": 1305,
///         "users_count": 5781486,
///         "wines_count": 577325,
///         "wineries_count": 67692,
///         "most_used_grapes": [
///             {
///                 "id": 14,
///                 "name": "Pinot Noir",
///                 "seo_name": "pinot-noir",
///                 "has_detailed_info": true,
///                 "wines_count": 572334
///             },
///             {
///                 "id": 5,
///                 "name": "Chardonnay",
///                 "seo_name": "chardonnay",
///                 "has_detailed_info": true,
///                 "wines_count": 604208
///             },
///             {
///                 "id": 10,
///                 "name": "Merlot",
///                 "seo_name": "merlot",
///                 "has_detailed_info": true,
///                 "wines_count": 566719
///             }
///         ]
///     },
///     "parent_id": 777,
///     "background_image": null,
///     "class": {
///         "typecast_map": {
///             "background_image": {},
///             "class": {}
///         }
///     },
///     "statistics": {
///         "wineries_count": 77,
///         "wines_count": 1060,
///         "sub_regions_count": 0,
///         "parent_regions_count": 2
///     }
/// },
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