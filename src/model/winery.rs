use serde::Deserialize;

/// Sample response JSON
/// ```json
/// "winery": {
///     "id": 7829,
///     "name": "M. Chapoutier",
///     "seo_name": "m-chapoutier",
///     "status": 1,
///     "review_status": "Completed",
///     "background_image": null,
///     "statistics": {
///         "ratings_count": 278736,
///         "ratings_average": 3.8,
///         "labels_count": 2565215,
///         "wines_count": 218
///     }
/// },
/// ```
#[derive(Debug, PartialEq, Deserialize)]
struct Winery {
    /// Vivino ID
    id: u32,
    name: String,
    seo_name: String,
    status: u8,
    review_status: String,
    background_image: Option<String>,
    statistics: WineryStatistics,
}
#[derive(Debug, PartialEq, Deserialize)]
struct WineryStatistics {
    ratings_count: u32,
    ratings_average: f32,
    labels_count: u32,
    wines_count: u32,
}