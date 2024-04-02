/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde_json::Value;
use std::collections::HashMap;

use crate::propelauth::token_models::OrgRoleStructure;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserInOrg {
    #[serde(rename = "org_id")]
    pub org_id: String,
    #[serde(rename = "org_name")]
    pub org_name: String,
    #[serde(rename = "org_metadata")]
    pub org_metadata: HashMap<String, Value>,
    #[serde(rename = "org_role_structure")]
    pub org_role_structure: OrgRoleStructure,
    #[serde(rename = "user_role")]
    pub user_role: String,
    #[serde(rename = "inherited_user_roles_plus_current_role")]
    pub inherited_user_roles_plus_current_role: Vec<String>,
    #[serde(rename = "user_permissions")]
    pub user_permissions: Vec<String>,
    #[serde(rename = "additional_roles")]
    pub additional_roles: Vec<String>,
}
