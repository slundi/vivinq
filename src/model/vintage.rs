use serde::Deserialize;

use super::{grape::Grape, image::Image, wine::Wine};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Vintage {
    pub id: u32,
    pub name: String,
    pub seo_name: String,
    pub statistics: VintageStatistics,
    pub image: Image,
    //not sure
    pub organic_certification_id: Option<String>,
    pub certified_biodynamic: Option<String>,
    pub wine: Wine,
    pub year: u16,
    pub grapes: Option<Vec<Grape>>,
    pub has_valid_ratings: bool,
}

/// Sample JSON:
/// ```json
/// {
///     "status": "Normal",
///     "ratings_count": 25,
///     "ratings_average": 4.1,
///     "labels_count": 166,
///     "reviews_count": 13
/// }
/// ```
#[derive(Debug, PartialEq, Deserialize)]
pub struct VintageStatistics {
    pub status: String,
    pub ratings_count: u32,
    pub ratings_average: f32,
    pub reviews_count: u32,
    pub labels_count: u16,
}


#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_deserialize_vintage() {
        let json ="{
            \"id\": 14780847,
            \"seo_name\": \"m-chapoutier-domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages-2013\",
            \"name\": \"M. Chapoutier Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages 2013\",
            \"statistics\": {  \"status\": \"Normal\", \"ratings_count\": 25, \"ratings_average\": 4.1, \"labels_count\": 166, \"reviews_count\": 13 },
            \"organic_certification_id\": null,
            \"certified_biodynamic\": null,
            \"image\": {
                \"location\": \"///images.vivino.com/labels/tXgj1w4zQFS48NmR074CXg.jpg\",
                \"variations\": { \"large\": \"///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_375x500.jpg\", \"medium\": \"///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_150x200.jpg\", \"medium_square\": \"///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_150x150.jpg\", \"small_square\": \"///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_80x80.jpg\" }
            },
            \"wine\": {
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
                        \"code\": \"fr\", \"name\": \"France\", \"native_name\": \"France\", \"seo_name\": \"france\",
                        \"currency\": { \"code\": \"EUR\", \"name\": \"Euros\", \"prefix\": \"€\", \"suffix\": null },
                        \"regions_count\": 1305, \"users_count\": 5781486, \"wines_count\": 577325, \"wineries_count\": 67692,
                        \"most_used_grapes\": [
                            { \"id\": 14, \"name\": \"Pinot Noir\", \"seo_name\": \"pinot-noir\", \"has_detailed_info\": true, \"wines_count\": 572334 },
                            { \"id\": 5, \"name\": \"Chardonnay\", \"seo_name\": \"chardonnay\", \"has_detailed_info\": true, \"wines_count\": 604208 },
                            { \"id\": 10, \"name\": \"Merlot\", \"seo_name\": \"merlot\", \"has_detailed_info\": true, \"wines_count\": 566719 }
                        ]
                    },
                    \"parent_id\": 777,
                    \"background_image\": null,
                    \"class\": { \"typecast_map\": { \"background_image\": {}, \"class\": {} } },
                    \"statistics\": { \"wineries_count\": 77, \"wines_count\": 1060, \"sub_regions_count\": 0, \"parent_regions_count\": 2 }
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
            },
            \"year\": 2013,
            \"grapes\": null,
            \"has_valid_ratings\": true
        }";
        let v: crate::model::vintage::Vintage = serde_json::from_str(json).unwrap();
        assert!(v.id == 14780847);
        assert!(v.name == *"M. Chapoutier Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages 2013");
        assert!(v.seo_name == *"m-chapoutier-domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages-2013");
        assert!(v.statistics.status == *"Normal");
        assert!(v.statistics.ratings_count == 25);
        assert!(v.statistics.ratings_average == 4.1);
        assert!(v.statistics.labels_count == 166);
        assert!(v.statistics.reviews_count == 13);
        assert!(v.organic_certification_id.is_none());
        assert!(v.certified_biodynamic.is_none());
        assert!(v.year == 2013);
        assert!(v.grapes.is_none());
        assert!(v.has_valid_ratings);
    }
}