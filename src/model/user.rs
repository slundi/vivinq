use serde::Deserialize;

use super::image::Image;

#[derive(Debug, PartialEq, Deserialize)]
pub struct User {
    pub id: u32,
    pub seo_name: String,
    pub alias: String,
    pub is_featured: bool,
    pub visibility: String,
    pub image: Image,
    pub statistics: UserStatistics,
    pub language: String,
    pub background_image: Image,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct UserStatistics {
    pub followers_count: u32,
    pub followings_count: u16,
    pub ratings_count: u16,
    pub ratings_sum: f32,
    pub reviews_count: u16,
    pub purchase_order_count: u16,
}

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_deserialize_user() {
        let json ="{
            \"id\": 21471450,
            \"seo_name\": \"fabien.cahuzac\",
            \"alias\": \"Fabien Cahuzac\",
            \"is_featured\": false,
            \"visibility\": \"all\",
            \"image\": {
                \"location\": \"///images.vivino.com/avatars/default_user.png\",
                \"variations\": null
            },
            \"statistics\": {
                \"followers_count\": 20,
                \"followings_count\": 7,
                \"ratings_count\": 519,
                \"ratings_sum\": 1880.3,
                \"reviews_count\": 509,
                \"purchase_order_count\": 0
            },
            \"language\": \"fr\",
            \"background_image\": {
                \"location\": \"///images.vivino.com/users/backgrounds/default_1.jpg\",
                \"variations\": {
                    \"large\": \"///images.vivino.com/users/backgrounds/default_1_1200x400.jpg\",
                    \"medium\": \"///images.vivino.com/users/backgrounds/default_1_600x200.jpg\",
                    \"small\": \"///images.vivino.com/users/backgrounds/default_1_140x60.jpg\"
                }
            }
        }";
        let u: crate::model::user::User = serde_json::from_str(json).unwrap();
        assert!(u.id == 21471450);
        assert!(u.seo_name == *"fabien.cahuzac");
        assert!(u.alias == *"Fabien Cahuzac");
        assert!(!u.is_featured);
        assert!(u.visibility == *"all");
        assert!(u.image.location == *"///images.vivino.com/avatars/default_user.png");
        assert!(u.image.variations.is_none());
        assert!(u.statistics.followers_count == 20);
        assert!(u.statistics.followings_count == 7);
        assert!(u.statistics.ratings_count == 519);
        assert!(u.statistics.ratings_sum == 1880.3);
        assert!(u.statistics.reviews_count == 509);
        assert!(u.statistics.purchase_order_count == 0);
        assert!(u.language == *"fr");
        assert!(u.background_image.location == *"///images.vivino.com/users/backgrounds/default_1.jpg");
        assert!(u.background_image.variations.is_some());
    }
}
