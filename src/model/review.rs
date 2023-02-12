use chrono::Utc;
use serde::Deserialize;

use super::{vintage::Vintage, activity::Activity, user::User};


#[derive(Debug, PartialEq, Deserialize)]
pub struct Review {
    /// Vivino ID
    pub id: u32,
    pub rating: f32,
    pub note: String,
    pub language: String,
    pub created_at: chrono::DateTime<Utc>,
    pub aggregated: bool,
    pub user: User,
    pub vintage: Vintage,
    pub activity: Activity,
    pub tagged_note: String,
}
/// ```

#[cfg(test)]
mod tests {
    use chrono::{Utc, Datelike, Timelike};
    use serde_json;

    #[test]
    fn test_deserialize_review() {
        let json ="{
            \"id\": 181846181,
            \"rating\": 4,
            \"note\": \"Fantastique travail sur ce rouge de chez Chapoutier, une vraie merveille \nDes tannins bien fondus et de la puissance \nDu fruit noir, boisé mais assez rond\nÉnorme longueur en bouche, robe rouge foncée \nDu chocolat et de la cerise noire sur la finale \",
            \"language\": \"fr\",
            \"created_at\": \"2020-10-31T14:20:15.000Z\",
            \"aggregated\": true,
            \"user\": {
                \"id\": 21471450,
                \"seo_name\": \"fabien.cahuzac\",
                \"alias\": \"Fabien Cahuzac\",
                \"is_featured\": false,
                \"visibility\": \"all\",
                \"image\": { \"location\": \"///images.vivino.com/avatars/default_user.png\", \"variations\": null },
                \"statistics\": { \"followers_count\": 20, \"followings_count\": 7, \"ratings_count\": 519, \"ratings_sum\": 1880.3, \"reviews_count\": 509, \"purchase_order_count\": 0 },
                \"language\": \"fr\",
                \"background_image\": {
                    \"location\": \"///images.vivino.com/users/backgrounds/default_1.jpg\", \"variations\": { \"large\": \"///images.vivino.com/users/backgrounds/default_1_1200x400.jpg\", \"medium\": \"///images.vivino.com/users/backgrounds/default_1_600x200.jpg\", \"small\": \"///images.vivino.com/users/backgrounds/default_1_140x60.jpg\" }
                }
            },
            \"vintage\": {
                \"id\": 14780847,
                \"seo_name\": \"m-chapoutier-domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages-2013\",
                \"name\": \"M. Chapoutier Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages 2013\",
                \"statistics\": { \"status\": \"Normal\", \"ratings_count\": 25, \"ratings_average\": 4.1, \"labels_count\": 166, \"reviews_count\": 13 },
                \"organic_certification_id\": null,
                \"certified_biodynamic\": null,
                \"image\": {
                    \"location\": \"///images.vivino.com/labels/tXgj1w4zQFS48NmR074CXg.jpg\",
                    \"variations\": {
                        \"large\": \"///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_375x500.jpg\",
                        \"medium\": \"///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_150x200.jpg\",
                        \"medium_square\": \"///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_150x150.jpg\",
                        \"small_square\": \"///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_80x80.jpg\"
                    }
                },
                \"wine\": {
                    \"id\": 1267148,
                    \"name\": \"Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages\",
                    \"seo_name\": \"domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages\",
                    \"type_id\": 1,
                    \"vintage_type\": 0,
                    \"is_natural\": false,
                    \"region\": {
                        \"id\": 2448,
                        \"name\": \"Côtes du Roussillon Villages\",
                        \"name_en\": \"\",
                        \"seo_name\": \"cotes-du-roussillon-villages\",
                        \"country\": { \"code\": \"fr\", \"name\": \"France\", \"native_name\": \"France\", \"seo_name\": \"france\",
                            \"currency\": { \"code\": \"EUR\", \"name\": \"Euros\", \"prefix\": \"€\", \"suffix\": null },
                            \"regions_count\": 1305, \"users_count\": 5781486, \"wines_count\": 577325, \"wineries_count\": 67692,
                            \"most_used_grapes\": [
                                { \"id\": 14, \"name\": \"Pinot Noir\", \"seo_name\": \"pinot-noir\", \"has_detailed_info\": true, \"wines_count\": 572334 },
                                { \"id\": 5, \"name\": \"Chardonnay\", \"seo_name\": \"chardonnay\", \"has_detailed_info\": true, \"wines_count\": 604208 }
                            ]
                        },
                        \"parent_id\": 777,
                        \"background_image\": null,
                        \"class\": { \"typecast_map\": { \"background_image\": {}, \"class\": {} } },
                        \"statistics\": { \"wineries_count\": 77, \"wines_count\": 1060, \"sub_regions_count\": 0, \"parent_regions_count\": 2
                        }
                    },
                    \"review_status\": 2,
                    \"winery\": {
                        \"id\": 7829,
                        \"name\": \"M. Chapoutier\",
                        \"seo_name\": \"m-chapoutier\",
                        \"status\": 1,
                        \"review_status\": \"Completed\",
                        \"background_image\": null,
                        \"statistics\": { \"ratings_count\": 278736, \"ratings_average\": 3.8, \"labels_count\": 2565215, \"wines_count\": 218
                        }
                    },
                    \"style\": null,
                    \"has_valid_ratings\": false
                },
                \"year\": 2013,
                \"grapes\": null,
                \"has_valid_ratings\": true
            },
            \"activity\": { \"id\": 484178004, \"statistics\": { \"likes_count\": 1, \"comments_count\": 0 } },
            \"tagged_note\": \"Fantastique travail sur ce rouge de chez Chapoutier, une vraie merveille \\nDes tannins bien fondus et de la puissance \\nDu fruit noir, boisé mais assez rond\\nÉnorme longueur en bouche, robe rouge foncée \\nDu chocolat et de la cerise noire sur la finale \"
        }";
        let r: crate::model::review::Review = serde_json::from_str(json).unwrap();
        assert!(r.id == 181846181);
        assert!(r.rating == 4.0);
        assert!(r.note == *"Fantastique travail sur ce rouge de chez Chapoutier, une vraie merveille \\nDes tannins bien fondus et de la puissance \\nDu fruit noir, boisé mais assez rond\\nÉnorme longueur en bouche, robe rouge foncée \\nDu chocolat et de la cerise noire sur la finale ");
        assert!(r.language == *"fr");
        assert!(r.created_at.offset() == &Utc);
        assert!(r.created_at.year() == 2020);
        assert!(r.created_at.month() == 10);
        assert!(r.created_at.day() == 31);
        assert!(r.created_at.hour() == 14);
        assert!(r.created_at.minute() == 20);
        assert!(r.created_at.second() == 15);
        assert!(r.aggregated);
        assert!(r.tagged_note == *"Fantastique travail sur ce rouge de chez Chapoutier, une vraie merveille \\nDes tannins bien fondus et de la puissance \\nDu fruit noir, boisé mais assez rond\\nÉnorme longueur en bouche, robe rouge foncée \\nDu chocolat et de la cerise noire sur la finale ");
        assert!(r.vintage.id == 14780847);
        assert!(r.activity.id == 484178004);
        assert!(r.activity.statistics.likes_count == 1);
        assert!(r.activity.statistics.comments_count == 0);
    }
}
