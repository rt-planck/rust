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
pub struct UserInOrg {
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "org_name")]
    pub org_name: String,
    #[serde(rename = "user_role")]
    pub user_role: String,
    #[serde(rename = "inherited_user_roles_plus_current_role")]
    pub inherited_user_roles_plus_current_role: Vec<String>,
    #[serde(rename = "user_permissions")]
    pub user_permissions: Vec<String>,
}

impl UserInOrg {
    pub fn new(org_id: String, org_name: String, user_role: String, inherited_user_roles_plus_current_role: Vec<String>, user_permissions: Vec<String>) -> UserInOrg {
        UserInOrg {
            org_id,
            org_name,
            user_role,
            inherited_user_roles_plus_current_role,
            user_permissions,
        }
    }
}


