use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    permission::{PermissionsList, PermissionsOverwrite},
    role::{ExistingRole, RolesList},
};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PermissionOverwriteType {
    Role = 0,
    // Member = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwritesRequest {
    #[serde(rename = "id")]
    pub role_or_member_id: String,
    #[serde(rename = "type")]
    pub _type: PermissionOverwriteType,
    pub allow: String,
    pub deny: String,
}

impl PermissionOverwritesRequest {
    pub fn from(overwrites: &PermissionsOverwrite, roles: &RolesList<ExistingRole>) -> Self {
        let role = roles
            .find_by_name(&overwrites.name)
            .unwrap_or_else(|| panic!("No role found for name {}", &overwrites.name));

        Self {
            _type: PermissionOverwriteType::Role,
            role_or_member_id: role.id.clone(),
            allow: overwrites.allow.code(),
            deny: overwrites.deny.code(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwritesResponse {
    #[serde(rename = "id")]
    pub role_or_member_id: String,
    #[serde(rename = "type")]
    pub _type: u8,
    pub allow: String,
    pub deny: String,
}

impl PermissionOverwritesResponse {
    pub fn _try_into(
        &self,
        roles: &RolesList<ExistingRole>,
    ) -> Result<PermissionsOverwrite, String> {
        if self._type != 0 {
            return Err(format!(
                "Unsupported permissions overwrite type {}",
                self._type
            ));
        };

        Ok(PermissionsOverwrite {
            name: roles.find_by_id(&self.role_or_member_id).name.clone(),
            allow: PermissionsList::from(self.allow.as_str()),
            deny: PermissionsList::from(self.deny.as_str()),
        })
    }
}
