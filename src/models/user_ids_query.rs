/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserIdsQuery {
    #[serde(rename = "user_ids")]
    pub user_ids: Vec<String>,
}

impl UserIdsQuery {
    pub fn new(user_ids: Vec<String>) -> UserIdsQuery {
        UserIdsQuery {
            user_ids,
        }
    }
}


