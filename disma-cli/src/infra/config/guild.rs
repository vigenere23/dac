use serde::{Deserialize, Serialize};

use disma::{
    category::{AwaitingCategory, CategoriesList},
    channel::{AwaitingChannel, ChannelsList},
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, RolesList},
    utils::vec::Compress,
};

use super::{category::CategoryConfig, channel::ChannelConfig, role::RoleConfig};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GuildConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<RoleConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<CategoryConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<Vec<ChannelConfig>>,
}

impl From<&ExistingGuild> for GuildConfig {
    fn from(guild: &ExistingGuild) -> Self {
        let roles: Vec<RoleConfig> = guild.roles.items().iter().map(|role| role.into()).collect();

        let categories: Vec<CategoryConfig> = guild
            .categories
            .items()
            .iter()
            .map(CategoryConfig::from)
            .collect();

        let channels: Vec<ChannelConfig> = guild
            .channels
            .items()
            .iter()
            .map(|channel| channel.into())
            .collect();

        Self {
            roles: roles.compress(),
            categories: categories.compress(),
            channels: channels.compress(),
        }
    }
}

impl Into<AwaitingGuild> for GuildConfig {
    fn into(self) -> AwaitingGuild {
        let roles: Vec<AwaitingRole> = self
            .roles
            .unwrap_or_default()
            .into_iter()
            .map(|role_config| role_config.into())
            .collect();

        let roles_list = RolesList::from(roles);

        let categories: Vec<AwaitingCategory> = self
            .categories
            .unwrap_or_default()
            .into_iter()
            .map(|category| category.into(&roles_list))
            .collect();

        let categories_list = CategoriesList::from(categories);

        let channels: Vec<AwaitingChannel> = self
            .channels
            .unwrap_or_default()
            .into_iter()
            .map(|channel| channel.into(&roles_list, &categories_list))
            .collect();

        let channels_list = ChannelsList::from(channels);

        AwaitingGuild {
            roles: roles_list,
            categories: categories_list,
            channels: channels_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        category::CategoriesList,
        channel::ChannelsList,
        guild::{AwaitingGuild, ExistingGuild},
        role::RolesList,
    };

    use super::GuildConfig;

    #[test]
    pub fn nones_are_converted_to_empty_arrays() {
        let config = GuildConfig {
            roles: None,
            categories: None,
            channels: None,
        };

        let entity: AwaitingGuild = config.into();

        let expected_entity = AwaitingGuild {
            roles: RolesList::from(vec![]),
            categories: CategoriesList::from(vec![]),
            channels: ChannelsList::from(vec![]),
        };
        assert_eq!(entity, expected_entity);
    }

    #[test]
    pub fn empty_arrays_are_converted_to_nones() {
        let entity = ExistingGuild {
            roles: RolesList::from(vec![]),
            categories: CategoriesList::from(vec![]),
            channels: ChannelsList::from(vec![]),
        };

        let config = GuildConfig::from(&entity);

        let expected_config = GuildConfig {
            roles: None,
            categories: None,
            channels: None,
        };
        assert_eq!(config, expected_config);
    }
}
