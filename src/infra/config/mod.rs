use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::{
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, RolesList},
};

use self::role::{FullRoleConfig, RoleConfig, RoleConfigAssembler};

pub mod role;

#[derive(Serialize, Deserialize)]
pub struct GuildConfig {
    roles: Vec<RoleConfig>,
    templates: Option<TemplatesConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct TemplatesConfig {
    roles: Option<Vec<FullRoleConfig>>,
}

impl From<&ExistingGuild> for GuildConfig {
    fn from(guild: &ExistingGuild) -> Self {
        let roles = guild
            .roles
            .items()
            .iter()
            .map(|role| RoleConfig::Full(role.into()))
            .collect();

        Self {
            roles,
            templates: None,
        }
    }
}

// TODO is this used?
impl From<&AwaitingGuild> for GuildConfig {
    fn from(guild: &AwaitingGuild) -> Self {
        let roles = guild
            .roles
            .items()
            .iter()
            .map(|role| RoleConfig::Full(role.into()))
            .collect();

        Self {
            roles,
            templates: None,
        }
    }
}

pub struct GuildConfigAssembler {
    role_assembler: Arc<RoleConfigAssembler>,
}

impl GuildConfigAssembler {
    pub fn new(role_assembler: Arc<RoleConfigAssembler>) -> Self {
        Self { role_assembler }
    }

    pub fn to_awaiting(&self, config: &GuildConfig) -> AwaitingGuild {
        let role_templates = config
            .templates
            .as_ref()
            .and_then(|templates| templates.roles.as_ref());

        let roles: Vec<AwaitingRole> = config
            .roles
            .iter()
            .map(|role_config| self.role_assembler.to_awaiting(role_config, role_templates))
            .collect();

        AwaitingGuild {
            roles: RolesList::from(roles),
        }
    }
}
