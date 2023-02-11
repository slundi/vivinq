use serde::Deserialize;

/// Sample JSONs:
/// ```json
/// "background_image": {
///     "location": "///images.vivino.com/users/backgrounds/default_1.jpg",
///     "variations": {
///         "large": "///images.vivino.com/users/backgrounds/default_1_1200x400.jpg",
///         "medium": "///images.vivino.com/users/backgrounds/default_1_600x200.jpg",
///         "small": "///images.vivino.com/users/backgrounds/default_1_140x60.jpg"
///     }
/// }
/// ```
/// or
/// ```json
/// "image": {
///     "location": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_480x640.png",
///     "variations": {
///         "bottle_large": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x960.png",
///         "bottle_medium": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x600.png",
///         "bottle_medium_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_600x600.png",
///         "bottle_small": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x300.png",
///         "bottle_small_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_300x300.png",
///         "label": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_480x640.png",
///         "label_large": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_375x500.png",
///         "label_medium": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x200.png",
///         "label_medium_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x150.png",
///         "label_small_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_80x80.png",
///         "large": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_375x500.png",
///         "medium": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x200.png",
///         "medium_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x150.png",
///         "small_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_80x80.png"
///     }
/// },
/// ```
#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Image {
    pub location: String,
    pub variations: Option<Variations>
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Variations {
    pub large: Option<String>,
    pub medium: Option<String>,
    pub medium_square: Option<String>,
    pub small: Option<String>,
    pub small_square: Option<String>,
    pub bottle_large: Option<String>,
    pub bottle_medium: Option<String>,
    pub bottle_medium_square: Option<String>,
    pub bottle_small: Option<String>,
    pub bottle_small_square: Option<String>,
    pub label: Option<String>,
    pub label_large: Option<String>,
    pub label_medium: Option<String>,
    pub label_medium_square: Option<String>,
}

mod tests {
    #[test]
    fn test_deserialize_image() {
        let json ="{
                     \"location\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_480x640.png\",
                     \"variations\": {
                         \"bottle_large\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x960.png\",
                         \"bottle_medium\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x600.png\",
                         \"bottle_medium_square\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_600x600.png\",
                         \"bottle_small\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x300.png\",
                         \"bottle_small_square\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_300x300.png\",
                         \"label\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_480x640.png\",
                         \"label_large\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_375x500.png\",
                         \"label_medium\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x200.png\",
                         \"label_medium_square\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x150.png\",
                         \"label_small_square\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_80x80.png\",
                         \"large\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_375x500.png\",
                         \"medium\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x200.png\",
                         \"medium_square\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x150.png\",
                         \"small_square\": \"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_80x80.png\"
                     }
                 }";
        let image: crate::model::image::Image = serde_json::from_str(json).unwrap();
        assert!(image.location == *"///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_480x640.png");
        let variations = image.variations.unwrap();
        assert!(variations.small.is_none());
        assert!(variations.medium == Some(String::from("///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x200.png")));
        assert!(variations.large == Some(String::from("///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_375x500.png")));
        assert!(variations.label == Some(String::from("///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_480x640.png")));
        // ....
    }

    #[test]
    fn test_deserialize_background_image() {
        let json = "{ \"location\": \"///images.vivino.com/users/backgrounds/default_1.jpg\", \"variations\": { \
                 \"large\": \"///images.vivino.com/users/backgrounds/default_1_1200x400.jpg\", \
                 \"medium\": \"///images.vivino.com/users/backgrounds/default_1_600x200.jpg\", \
                 \"small\": \"///images.vivino.com/users/backgrounds/default_1_140x60.jpg\" \
             } }";
        let image: crate::model::image::Image = serde_json::from_str(json).unwrap();
        assert!(image.location == *"///images.vivino.com/users/backgrounds/default_1.jpg");
        let variations = image.variations.unwrap();
        assert!(variations.large == Some(String::from("///images.vivino.com/users/backgrounds/default_1_1200x400.jpg")));
        assert!(variations.medium == Some(String::from("///images.vivino.com/users/backgrounds/default_1_600x200.jpg")));
        assert!(variations.small == Some(String::from("///images.vivino.com/users/backgrounds/default_1_140x60.jpg")));
        assert!(variations.medium_square.is_none());
    }
}
