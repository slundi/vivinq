use serde::Deserialize;

use super::currency::Currency;
use super::grape::Grape;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub seo_name: String,
    pub currency: Currency,
    pub regions_count: u16,
    pub users_count: u32,
    pub wines_count: u32,
    pub wineries_count: u32,
    pub most_used_grapes: Option<Vec<Grape>>,
}


#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_deserialize_country() {
        let json = "{
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
        }";
        let c: crate::model::country::Country = serde_json::from_str(json).unwrap();
        assert!(c.code == *"fr");
        assert!(c.name == *"France");
        assert!(c.native_name == *"France");
        assert!(c.seo_name == *"france");
        assert!(c.regions_count == 1305);
        assert!(c.users_count == 5781486);
        assert!(c.wines_count == 577325);
        assert!(c.wineries_count == 67692);
        assert!(c.most_used_grapes.unwrap().len() == 2);
    }
}
