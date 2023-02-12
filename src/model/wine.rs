use serde::Deserialize;

use super::{region::Region, image::Image, winery::Winery};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Wine {
    pub id: u32,
    pub name: String,
    pub seo_name: String,
    pub type_id: u8,
    pub vintage_type: u8,
    pub is_natural: bool,
    pub region: Region,
    pub background_image: Option<Image>,
    pub review_status: u8,
    pub winery: Winery,
    /// TODO: check style with other wine
    pub style: Option<String>,
    pub has_valid_ratings: bool,
}

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_deserialize_wine() {
        let json ="{
            \"id\": 1267148,
            \"name\": \"Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages\",
            \"seo_name\": \"domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages\",
            \"type_id\": 1,
            \"vintage_type\": 0,
            \"is_natural\": false,
            \"region\": {
                \"id\": 2448,
                \"name\": \"Côtes du Roussillon Villages\",
                \"name_en\": \"\",
                \"seo_name\": \"cotes-du-roussillon-villages\",
                \"country\": {
                    \"code\": \"fr\",
                    \"name\": \"France\",
                    \"native_name\": \"France\",
                    \"seo_name\": \"france\",
                    \"currency\": { \"code\": \"EUR\", \"name\": \"Euros\", \"prefix\": \"€\", \"suffix\": null },
                    \"regions_count\": 1305,
                    \"users_count\": 5781486,
                    \"wines_count\": 577325,
                    \"wineries_count\": 67692,
                    \"most_used_grapes\": [
                        { \"id\": 14, \"name\": \"Pinot Noir\", \"seo_name\": \"pinot-noir\", \"has_detailed_info\": true, \"wines_count\": 572334 },
                        { \"id\": 5, \"name\": \"Chardonnay\", \"seo_name\": \"chardonnay\", \"has_detailed_info\": true, \"wines_count\": 604208 },
                        { \"id\": 10, \"name\": \"Merlot\", \"seo_name\": \"merlot\", \"has_detailed_info\": true, \"wines_count\": 566719 }
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
            },
            \"review_status\": 2,
            \"winery\": {
                \"id\": 7829,
                \"name\": \"M. Chapoutier\",
                \"seo_name\": \"m-chapoutier\",
                \"status\": 1,
                \"review_status\": \"Completed\",
                \"background_image\": null,
                \"statistics\": { \"ratings_count\": 278736, \"ratings_average\": 3.8, \"labels_count\": 2565215, \"wines_count\": 218 }
            },
            \"style\": null,
            \"has_valid_ratings\": false
        }";
        let w: crate::model::wine::Wine = serde_json::from_str(json).unwrap();
        assert!(w.id == 1267148);
        assert!(w.name == *"Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages");
        assert!(w.seo_name == *"domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages");
        assert!(w.type_id == 1);
        assert!(w.vintage_type == 0);
        assert!(!w.is_natural);
        assert!(w.review_status == 2);
        assert!(w.style.is_none());
        assert!(!w.has_valid_ratings);
    }
}
