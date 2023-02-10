use serde::Deserialize;

/// Example review Vivino response:
/// ```json
/// {
/// "reviews": [
///     {
///         "id": 181846181,
///         "rating": 4,
///         "note": "Fantastique travail sur ce rouge de chez Chapoutier, une vraie merveille \nDes tannins bien fondus et de la puissance \nDu fruit noir, boisé mais assez rond\nÉnorme longueur en bouche, robe rouge foncée \nDu chocolat et de la cerise noire sur la finale ",
///         "language": "fr",
///         "created_at": "2020-10-31T14:20:15.000Z",
///         "aggregated": true,
///         "user": {
///             "id": 21471450,
///             "seo_name": "fabien.cahuzac",
///             "alias": "Fabien Cahuzac",
///             "is_featured": false,
///             "visibility": "all",
///             "image": {
///                 "location": "///images.vivino.com/avatars/default_user.png",
///                 "variations": null
///             },
///             "statistics": {
///                 "followers_count": 20,
///                 "followings_count": 7,
///                 "ratings_count": 519,
///                 "ratings_sum": 1880.3,
///                 "reviews_count": 509,
///                 "purchase_order_count": 0
///             },
///             "language": "fr",
///             "background_image": {
///                 "location": "///images.vivino.com/users/backgrounds/default_1.jpg",
///                 "variations": {
///                     "large": "///images.vivino.com/users/backgrounds/default_1_1200x400.jpg",
///                     "medium": "///images.vivino.com/users/backgrounds/default_1_600x200.jpg",
///                     "small": "///images.vivino.com/users/backgrounds/default_1_140x60.jpg"
///                 }
///             }
///         },
///         "vintage": {
///             "id": 14780847,
///             "seo_name": "m-chapoutier-domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages-2013",
///             "name": "M. Chapoutier Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages 2013",
///             "statistics": {
///                 "status": "Normal",
///                 "ratings_count": 25,
///                 "ratings_average": 4.1,
///                 "labels_count": 166,
///                 "reviews_count": 13
///             },
///             "organic_certification_id": null,
///             "certified_biodynamic": null,
///             "image": {
///                 "location": "///images.vivino.com/labels/tXgj1w4zQFS48NmR074CXg.jpg",
///                 "variations": {
///                     "large": "///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_375x500.jpg",
///                     "medium": "///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_150x200.jpg",
///                     "medium_square": "///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_150x150.jpg",
///                     "small_square": "///images.vivino.com/thumbs/tXgj1w4zQFS48NmR074CXg_80x80.jpg"
///                 }
///             },
///             "wine": {
///                 "id": 1267148,
///                 "name": "Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages",
///                 "seo_name": "domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages",
///                 "type_id": 1,
///                 "vintage_type": 0,
///                 "is_natural": false,
///                 "region": {
///                     "id": 2448,
///                     "name": "Côtes du Roussillon Villages",
///                     "name_en": "",
///                     "seo_name": "cotes-du-roussillon-villages",
///                     "country": {
///                         "code": "fr",
///                         "name": "France",
///                         "native_name": "France",
///                         "seo_name": "france",
///                         "currency": {
///                             "code": "EUR",
///                             "name": "Euros",
///                                 "prefix": "€",
///                                 "suffix": null
///                         },
///                         "regions_count": 1305,
///                         "users_count": 5781486,
///                         "wines_count": 577325,
///                         "wineries_count": 67692,
///                         "most_used_grapes": [
///                             {
///                                 "id": 14,
///                                 "name": "Pinot Noir",
///                                 "seo_name": "pinot-noir",
///                                 "has_detailed_info": true,
///                                 "wines_count": 572334
///                             },
///                             {
///                                 "id": 5,
///                                 "name": "Chardonnay",
///                                 "seo_name": "chardonnay",
///                                 "has_detailed_info": true,
///                                 "wines_count": 604208
///                             }
///                         ]
///                     },
///                     "parent_id": 777,
///                     "background_image": null,
///                     "class": {
///                         "typecast_map": {
///                             "background_image": {},
///                             "class": {}
///                         }
///                     },
///                     "statistics": {
///                         "wineries_count": 77,
///                         "wines_count": 1060,
///                         "sub_regions_count": 0,
///                         "parent_regions_count": 2
///                     }
///                 },
///                 "review_status": 2,
///                 "winery": {
///                     "id": 7829,
///                     "name": "M. Chapoutier",
///                     "seo_name": "m-chapoutier",
///                     "status": 1,
///                     "review_status": "Completed",
///                     "background_image": null,
///                     "statistics": {
///                         "ratings_count": 278736,
///                         "ratings_average": 3.8,
///                         "labels_count": 2565215,
///                         "wines_count": 218
///                     }
///                 },
///                 "style": null,
///                 "has_valid_ratings": false
///             },
///             "year": 2013,
///             "grapes": null,
///             "has_valid_ratings": true
///         },
///         "activity": {
///             "id": 484178004,
///             "statistics": {
///                 "likes_count": 1,
///                 "comments_count": 0
///             }
///         },
///         "tagged_note": "Fantastique travail sur ce rouge de chez Chapoutier, une vraie merveille \nDes tannins bien fondus et de la puissance \nDu fruit noir, boisé mais assez rond\nÉnorme longueur en bouche, robe rouge foncée \nDu chocolat et de la cerise noire sur la finale "
///     },
///     {
///         "id": 96979985,
///         "rating": 2.5,
///         "note": "Très déçue surtout pour ce prix \nIl est rond et se boit bien mais plat et sans relief \nTrès peu d intérêt à dépenser autant pour ce vin",
///         "language": "fr",
///         "created_at": "2018-06-09T09:38:35.000Z",
///         "aggregated": true,
///         "user": {
///             "id": 27812955,
///             "seo_name": "marie-david1",
///             "alias": "Marie David",
///             "is_featured": false,
///             "visibility": "all",
///             "image": {
///                 "location": "///images.vivino.com/avatars/default_user.png",
///                 "variations": null
///             },
///             "statistics": {
///                 "followers_count": 0,
///                 "followings_count": 0,
///                 "ratings_count": 16,
///                 "ratings_sum": 58,
///                 "reviews_count": 16,
///                 "purchase_order_count": 3
///             },
///             "language": "fr",
///             "background_image": {
///                 "location": "///images.vivino.com/users/backgrounds/default_1.jpg",
///                 "variations": {
///                     "large": "///images.vivino.com/users/backgrounds/default_1_1200x400.jpg",
///                     "medium": "///images.vivino.com/users/backgrounds/default_1_600x200.jpg",
///                     "small": "///images.vivino.com/users/backgrounds/default_1_140x60.jpg"
///                 }
///             }
///         },
///         "vintage": {
///             "id": 14780847,
///             "seo_name": "m-chapoutier-domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages-2013",
///             "name": "M. Chapoutier Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages 2013",
///             "statistics": {
///                 "status": "Normal",
///                 "ratings_count": 25,
///                 "ratings_average": 4.1,
///                 "labels_count": 166,
///                 "reviews_count": 13
///             },
///             "organic_certification_id": null,
///             "certified_biodynamic": null,
///             "image": {
///                 "location": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_480x640.png",
///                 "variations": {
///                     "bottle_large": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x960.png",
///                     "bottle_medium": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x600.png",
///                     "bottle_medium_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_600x600.png",
///                     "bottle_small": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_x300.png",
///                     "bottle_small_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pb_300x300.png",
///                     "label": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_480x640.png",
///                     "label_large": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_375x500.png",
///                     "label_medium": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x200.png",
///                     "label_medium_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x150.png",
///                     "label_small_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_80x80.png",
///                     "large": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_375x500.png",
///                     "medium": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x200.png",
///                     "medium_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_150x150.png",
///                     "small_square": "///images.vivino.com/thumbs/7phdZm64SHiWwH8wwCpedQ_pl_80x80.png"
///                 }
///             },
///             "wine": {
///                 "id": 1267148,
///                 "name": "Domaine de Bila-Haut V.I.T (Visitare Interiore Terrae) Côtes du Roussillon Villages",
///                 "seo_name": "domaine-de-bila-haut-v-i-t-visitare-interiore-terrae-cotes-du-roussillon-villages",
///                 "type_id": 1,
///                 "vintage_type": 0,
///                 "is_natural": false,
///                 "region": {
///                     "id": 2448,
///                     "name": "Côtes du Roussillon Villages",
///                     "name_en": "",
///                     "seo_name": "cotes-du-roussillon-villages",
///                     "country": {
///                         "code": "fr",
///                         "name": "France",
///                         "native_name": "France",
///                         "seo_name": "france",
///                         "currency": {
///                             "code": "EUR",
///                             "name": "Euros",
///                             "prefix": "€",
///                             "suffix": null
///                         },
///                         "regions_count": 1305,
///                         "users_count": 5781486,
///                         "wines_count": 577325,
///                         "wineries_count": 67692,
///                         "most_used_grapes": [
///                             {
///                                 "id": 14,
///                                 "name": "Pinot Noir",
///                                 "seo_name": "pinot-noir",
///                                 "has_detailed_info": true,
///                                 "wines_count": 572334
///                             },
///                             {
///                                     "id": 5,
///                                 "name": "Chardonnay",
///                                 "seo_name": "chardonnay",
///                                 "has_detailed_info": true,
///                                 "wines_count": 604208
///                             },
///                             {
///                                 "id": 10,
///                                 "name": "Merlot",
///                                 "seo_name": "merlot",
///                                 "has_detailed_info": true,
///                                 "wines_count": 566719
///                             }
///                         ]
///                     },
///                     "parent_id": 777,
///                     "background_image": null,
///                     "class": {
///                         "typecast_map": {
///                             "background_image": {},
///                             "class": {}
///                         }
///                     },
///                     "statistics": {
///                         "wineries_count": 77,
///                         "wines_count": 1060,
///                         "sub_regions_count": 0,
///                         "parent_regions_count": 2
///                     }
///                 },
///                 "review_status": 2,
///                 "winery": {
///                     "id": 7829,
///                     "name": "M. Chapoutier",
///                     "seo_name": "m-chapoutier",
///                     "status": 1,
///                     "review_status": "Completed",
///                     "background_image": null,
///                     "statistics": {
///                         "ratings_count": 278736,
///                         "ratings_average": 3.8,
///                         "labels_count": 2565215,
///                         "wines_count": 218
///                     }
///                 },
///                 "style": null,
///                 "has_valid_ratings": false
///             },
///             "year": 2013,
///             "grapes": null,
///             "has_valid_ratings": true
///         },
///         "activity": {
///             "id": 256657828,
///             "statistics": {
///                 "likes_count": 0,
///                 "comments_count": 0
///             }
///         },
///         "tagged_note": "Très déçue surtout pour ce prix \nIl est rond et se boit bien mais plat et sans relief \nTrès peu d intérêt à dépenser autant pour ce vin"
///     }
///     ]
/// }
/// ```
#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Review {
    /// Vivino ID
    id: u32,
    tagged_note: String,
}
