use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Variation {
    large: Option<String>,
    medium: Option<String>,
    medium_square: Option<String>,
    small: Option<String>,
    small_square: Option<String>,
    bottle_large: Option<String>,
    bottle_medium: Option<String>,
    bottle_medium_square: Option<String>,
    bottle_small: Option<String>,
    bottle_small_square: Option<String>,
    label: Option<String>,
    label_large: Option<String>,
    label_medium: Option<String>,
    label_medium_square: Option<String>,
}

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
    location: String,
    variations: Option<Variation>
}
