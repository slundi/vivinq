use serde::Deserialize;

use super::{country::Country, image::Image};

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Region {
    pub id: u32,
    pub name: String,
    pub name_en: String,
    pub seo_name: String,
    pub country: Country,
    pub parent_id: u32,
    pub background_image: Option<Image>,
    //pub class: Class,
    pub statistics: RegionStatistics,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct RegionStatistics {
    pub wineries_count: u32,
    pub wines_count: u32,
    pub sub_regions_count: u16,
    pub parent_regions_count: u16,
}

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_deserialize_region() {
        let json = "{
            \"id\": 2448,
            \"name\": \"Côtes du Roussillon Villages\",
            \"name_en\": \"\",
            \"seo_name\": \"cotes-du-roussillon-villages\",
            \"country\": {
                \"code\": \"fr\",
                \"name\": \"France\",
                \"native_name\": \"France\",
                \"seo_name\": \"france\",
                \"currency\": {
                    \"code\": \"EUR\",
                    \"name\": \"Euros\",
                    \"prefix\": \"€\",
                    \"suffix\": null
                },
                \"regions_count\": 1305,
                \"users_count\": 5781486,
                \"wines_count\": 577325,
                \"wineries_count\": 67692,
                \"most_used_grapes\": [
                    {
                        \"id\": 14,
                        \"name\": \"Pinot Noir\",
                        \"seo_name\": \"pinot-noir\",
                        \"has_detailed_info\": true,
                        \"wines_count\": 572334
                    },
                    {
                        \"id\": 5,
                        \"name\": \"Chardonnay\",
                        \"seo_name\": \"chardonnay\",
                        \"has_detailed_info\": true,
                        \"wines_count\": 604208
                    }
                ]
            },
            \"parent_id\": 777,
            \"background_image\": null,
            \"class\": {
                \"typecast_map\": {
                    \"background_image\": {},
                    \"class\": {}
                }
            },
            \"statistics\": {
                \"wineries_count\": 77,
                \"wines_count\": 1060,
                \"sub_regions_count\": 0,
                \"parent_regions_count\": 2
            }
        }";
        let r: crate::model::region::Region = serde_json::from_str(json).unwrap();
        assert!(r.id == 2448);
        assert!(r.name == *"Côtes du Roussillon Villages");
        assert!(r.name_en.is_empty());
        assert!(r.seo_name == *"cotes-du-roussillon-villages");
        assert!(r.parent_id == 777);
        assert!(r.statistics.wineries_count == 77);
        assert!(r.statistics.wines_count == 1060);
        assert!(r.statistics.sub_regions_count == 0);
        assert!(r.statistics.parent_regions_count == 2);
    }
}
