use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Currency {
    code: String,
    name: String,
    prefix: Option<String>,
    suffix: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_deserialize_activity() {
        let json =
            "{ \"code\": \"EUR\", \"name\": \"Euros\", \"prefix\": \"€\", \"suffix\": null }";
        let c: crate::model::currency::Currency = serde_json::from_str(json).unwrap();
        assert!(c.code == "EUR");
        assert!(c.name == "Euros");
        assert!(c.prefix == Some(String::from("€")));
        assert!(c.suffix.is_none());
    }
}
