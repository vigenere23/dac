use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, Channel, ExistingChannel},
    core::commands::{CommandRef, DeleteChannel, UpdateChannel},
    role::{ExistingRole, RolesList},
};

pub trait ExtraChannelsStrategy {
    fn _type(&self) -> ExtraChannelsStrategyType;
    fn handle_extra_channel(
        &self,
        extra_channel: &ExistingChannel,
        commands: &mut Vec<CommandRef>,
        awaiting_category: Option<&AwaitingCategory>,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    );
}

#[derive(Debug, PartialEq)]
pub enum ExtraChannelsStrategyType {
    Keep,
    Remove,
    OverwritePermissionsWithCategory,
}

impl Debug for dyn ExtraChannelsStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self._type())
    }
}

pub struct RemoveExtraChannels {}

impl ExtraChannelsStrategy for RemoveExtraChannels {
    fn _type(&self) -> ExtraChannelsStrategyType {
        ExtraChannelsStrategyType::Remove
    }

    fn handle_extra_channel(
        &self,
        extra_channel: &ExistingChannel,
        commands: &mut Vec<CommandRef>,
        _awaiting_category: Option<&AwaitingCategory>,
        _roles: &RolesList<ExistingRole>,
        _categories: &CategoriesList<ExistingCategory>,
    ) {
        let command = DeleteChannel::new(extra_channel.clone());
        commands.push(Arc::from(command));
    }
}

pub struct KeepExtraChannels {}

impl ExtraChannelsStrategy for KeepExtraChannels {
    fn _type(&self) -> ExtraChannelsStrategyType {
        ExtraChannelsStrategyType::Keep
    }

    fn handle_extra_channel(
        &self,
        _extra_channel: &ExistingChannel,
        _commands: &mut Vec<CommandRef>,
        _awaiting_category: Option<&AwaitingCategory>,
        _roles: &RolesList<ExistingRole>,
        _categories: &CategoriesList<ExistingCategory>,
    ) {
    }
}

pub struct SyncExtraChannelsPermissions {}

impl ExtraChannelsStrategy for SyncExtraChannelsPermissions {
    fn _type(&self) -> ExtraChannelsStrategyType {
        ExtraChannelsStrategyType::OverwritePermissionsWithCategory
    }

    fn handle_extra_channel(
        &self,
        extra_channel: &ExistingChannel,
        commands: &mut Vec<CommandRef>,
        awaiting_category: Option<&AwaitingCategory>,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) {
        if let Some(category) = awaiting_category {
            let awaiting_channel = AwaitingChannel {
                name: extra_channel.name().to_string(),
                topic: extra_channel.topic.clone(),
                channel_type: extra_channel.channel_type.clone(),
                category: Some(category.clone()),
                overwrites: category.overwrites.clone(),
            };

            let command = UpdateChannel::new(
                extra_channel.clone(),
                awaiting_channel,
                roles.clone(),
                categories.clone(),
            );
            commands.push(Arc::from(command));
        } else {
            panic!("Category cannot be empty for overriding permissions overwrites of extra channel {}", extra_channel.name());
        }
    }
}