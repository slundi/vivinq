use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Activity {
    /// Vivino ID
    pub id: u32,
    pub statistics: ActivityStatistics,
}
#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct ActivityStatistics {
    pub likes_count: u32,
    pub comments_count: u32,
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::Activity;

    #[test]
    fn test_deserialize_activity() {
        let json =
            "{\"id\": 256657828, \"statistics\": { \"likes_count\": 0, \"comments_count\": 0 }}";
        let activity: Activity = serde_json::from_str(json).unwrap();
        assert!(activity.id == 256657828);
        assert!(activity.statistics.likes_count == 0);
        assert!(activity.statistics.comments_count == 0);
    }
}
