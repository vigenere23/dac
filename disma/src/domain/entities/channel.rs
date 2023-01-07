use crate::{
    category::{AwaitingCategory, Category, ExistingCategory},
    diff::base::{Diff, Differ},
    overwrites::PermissionsOverwritesList,
    role::{AwaitingRole, ExistingRole},
    utils::{misc::IfThen, option::OptionEq},
};

use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, PartialEq, Clone)]
pub enum ChannelType {
    TEXT,
    VOICE,
}

impl Differ<ChannelType> for ChannelType {
    fn diffs_with(&self, target: &ChannelType) -> Vec<Diff> {
        self.to_string().diffs_with(&target.to_string())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AwaitingChannel {
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<AwaitingCategory>,
    pub overwrites: PermissionsOverwritesList<AwaitingRole>,
}

#[derive(Debug, Clone)]
pub struct ExistingChannel {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<ExistingCategory>,
    pub overwrites: PermissionsOverwritesList<ExistingRole>,
}

impl PartialEq<AwaitingChannel> for ExistingChannel {
    fn eq(&self, other: &AwaitingChannel) -> bool {
        self.name == other.name
            && self.topic == other.topic
            && self.channel_type == other.channel_type
            && self.category.option_eq(&other.category)
            && self.overwrites == other.overwrites
    }
}

impl Differ<AwaitingChannel> for ExistingChannel {
    fn diffs_with(&self, awaiting: &AwaitingChannel) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.topic.diffs_with(&awaiting.topic).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("topic".into(), diffs)),
        );

        self.channel_type
            .diffs_with(&awaiting.channel_type)
            .if_then(
                |diffs| !diffs.is_empty(),
                |diffs| all_diffs.push(Diff::Update("channel_type".into(), diffs)),
            );

        self.category.diffs_with(&awaiting.category).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("category".into(), diffs)),
        );

        self.overwrites.diffs_with(&awaiting.overwrites).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("overwrites".into(), diffs)),
        );

        all_diffs
    }
}

pub trait Channel {
    fn name(&self) -> String;
    fn category_name(&self) -> Option<String>;
    fn channel_type(&self) -> ChannelType;

    fn unique_name(&self) -> String {
        format!(
            "{}:{} ({})",
            &self.category_name().unwrap_or_default(),
            &self.name(),
            &self.channel_type().to_string()
        )
    }
}

impl Channel for AwaitingChannel {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn category_name(&self) -> Option<String> {
        self.category.as_ref().map(|category| category.name())
    }

    fn channel_type(&self) -> ChannelType {
        self.channel_type.clone()
    }
}

impl Channel for ExistingChannel {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn category_name(&self) -> Option<String> {
        self.category.as_ref().map(|category| category.name())
    }

    fn channel_type(&self) -> ChannelType {
        self.channel_type.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct ChannelsList<C>
where
    C: Channel,
{
    items: Vec<C>,
}

impl<C> ChannelsList<C>
where
    C: Channel,
{
    pub fn find(
        &self,
        name: &str,
        _type: ChannelType,
        category_name: Option<String>,
    ) -> Option<&C> {
        self.items.iter().find(|channel| {
            channel.name() == name
                && channel.channel_type() == _type
                && channel.category_name() == category_name
        })
    }

    pub fn items(&self) -> &Vec<C> {
        &self.items
    }
}

impl<C> From<Vec<C>> for ChannelsList<C>
where
    C: Channel,
{
    fn from(items: Vec<C>) -> Self {
        Self { items }
    }
}

#[cfg(test)]
mod tests {
    mod channel_type {
        use std::str::FromStr;

        use crate::channel::ChannelType;

        #[test]
        fn can_format_to_string() {
            let formatted = ChannelType::TEXT.to_string();
            assert_eq!(formatted, "TEXT");
        }

        #[test]
        fn can_be_parsed_from_string() {
            let parsed = ChannelType::from_str("TEXT").unwrap();
            assert_eq!(parsed, ChannelType::TEXT);
        }

        #[test]
        fn given_invalid_string_it_cannot_parse_from_string() {
            let parsed = ChannelType::from_str("bullshit");
            assert!(parsed.is_err());
        }
    }
}