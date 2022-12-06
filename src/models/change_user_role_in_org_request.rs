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
pub struct ChangeUserRoleInOrgRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "role")]
    pub role: String,
}

impl ChangeUserRoleInOrgRequest {
    pub fn new(user_id: String, org_id: String, role: String) -> ChangeUserRoleInOrgRequest {
        ChangeUserRoleInOrgRequest {
            user_id,
            org_id,
            role,
        }
    }
}


