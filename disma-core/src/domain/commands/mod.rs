use std::sync::Arc;

use super::entities::guild::GuildCommander;

pub mod category;
pub mod roles;

pub trait GuildCommand {
    fn execute(&self, guild: Arc<dyn GuildCommander>);
    fn describe(&self) -> String;
}