use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Winery {
    /// Vivino ID
    pub id: u32,
    pub name: String,
    pub seo_name: String,
    pub status: u8,
    pub review_status: String,
    pub background_image: Option<String>,
    pub statistics: WineryStatistics,
}
#[derive(Debug, PartialEq, Deserialize)]
pub struct WineryStatistics {
    pub ratings_count: u32,
    pub ratings_average: f32,
    pub labels_count: u32,
    pub wines_count: u32,
}

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_deserialize_winery() {
        let json ="{
                \"id\": 7829,
                \"name\": \"M. Chapoutier\",
                \"seo_name\": \"m-chapoutier\",
                \"status\": 1,
                \"review_status\": \"Completed\",
                \"background_image\": null,
                \"statistics\": {
                    \"ratings_count\": 278736,
                    \"ratings_average\": 3.8,
                    \"labels_count\": 2565215,
                    \"wines_count\": 218
                }
            }";
        let w: crate::model::winery::Winery = serde_json::from_str(json).unwrap();
        assert!(w.id == 7829);
        assert!(w.name == *"M. Chapoutier");
        assert!(w.seo_name == *"m-chapoutier");
        assert!(w.status == 1);
        assert!(w.background_image.is_none());
        assert!(w.review_status == *"Completed");
        assert!(w.statistics.ratings_count == 278736);
        assert!(w.statistics.ratings_average == 3.8);
        assert!(w.statistics.labels_count == 2565215);
        assert!(w.statistics.wines_count == 218);
    }
}
