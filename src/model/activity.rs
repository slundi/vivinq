use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Activity {
    /// Vivino ID
    id: u32,
}
#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct ActivityStatistics {
    like_count: u32,
    comments_count: u32,
}
