use serde::Deserialize;

use super::currency::Currency;
use super::grape::Grape;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Country {
    code: String,
    name: String,
    native_name: String,
    currency: Currency,
    regions_count: u16,
    users_count: u32,
    wines_count: u32,
    wineries_count: u32,
    most_used_grapes_count: Option<Vec<Grape>>,
}