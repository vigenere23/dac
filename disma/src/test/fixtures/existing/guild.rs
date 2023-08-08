#[cfg(test)]
pub mod tests {
    use crate::{
        category::{CategoriesList, ExistingCategory},
        channel::{ChannelsList, ExistingChannel},
        guild::ExistingGuild,
        role::{ExistingRole, RolesList},
    };

    pub struct ExistingGuildFixture {
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
        channels: ChannelsList<ExistingChannel>,
    }

    impl ExistingGuildFixture {
        pub fn new() -> Self {
            Self {
                roles: RolesList::from(Vec::new()),
                categories: CategoriesList::from(Vec::new()),
                channels: ChannelsList::from(Vec::new()),
            }
        }

        pub fn with_role(mut self, role: ExistingRole) -> Self {
            self.roles.push(role);
            self
        }

        pub fn with_category(mut self, category: ExistingCategory) -> Self {
            self.categories.push(category);
            self
        }

        pub fn build(self) -> ExistingGuild {
            ExistingGuild {
                roles: self.roles,
                categories: self.categories,
                channels: self.channels,
            }
        }
    }
}
