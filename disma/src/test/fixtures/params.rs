#[cfg(test)]
pub mod tests {
    use crate::params::{
        category::CategoriesParamsList, channel::ChannelsParamsList, guild::GuildParams,
        role::RolesParamsList,
    };

    pub struct GuildParamsFixture {
        roles: RolesParamsList,
        categories: CategoriesParamsList,
        channels: ChannelsParamsList,
    }

    impl GuildParamsFixture {
        pub fn new() -> Self {
            Self {
                roles: RolesParamsList::default(),
                categories: CategoriesParamsList::default(),
                channels: ChannelsParamsList::default(),
            }
        }

        pub fn default() -> GuildParams {
            Self::new().build()
        }

        pub fn build(self) -> GuildParams {
            GuildParams {
                roles: self.roles,
                categories: self.categories,
                channels: self.channels,
            }
        }
    }
}
