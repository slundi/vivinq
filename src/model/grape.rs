use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Grape {
    pub id: u32,
    pub name: String,
    pub seo_name: String,
    pub has_detailed_info: bool,
    pub wines_count: u32,
}

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_deserialize_activity() {
        let json = "{
                \"id\": 14,
                \"name\": \"Pinot Noir\",
                \"seo_name\": \"pinot-noir\",
                \"has_detailed_info\": true,
                \"wines_count\": 572334
            }";
        let g: crate::model::grape::Grape = serde_json::from_str(json).unwrap();
        assert!(g.id == 14);
        assert!(g.name == *"Pinot Noir");
        assert!(g.seo_name == *"pinot-noir");
        assert!(g.has_detailed_info);
        assert!(g.wines_count == 572334);
    }
}
