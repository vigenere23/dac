use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    category::ExistingCategory,
    commands::{Command, CommandDescription, CommandEntity, CommandRef},
    guild::GuildCommanderRef,
};

#[deprecated = "Use core::commands::DeleteCategory command instead"]
pub struct DeleteCategory {
    category: ExistingCategory,
}

impl DeleteCategory {
    pub fn new(category: ExistingCategory) -> Self {
        Self { category }
    }
}

impl Command for DeleteCategory {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.delete_category(&self.category.id).unwrap();
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Delete(CommandEntity::Category, self.category.name.clone())
    }
}

pub trait ExtraCategoriesStrategy {
    fn _type(&self) -> ExtraCategoriesStrategyType;
    fn handle_extra_category(
        &self,
        extra_category: &ExistingCategory,
        commands: &mut Vec<CommandRef>,
    );
}

#[derive(Debug, PartialEq)]
pub enum ExtraCategoriesStrategyType {
    Keep,
    Remove,
}

impl Debug for dyn ExtraCategoriesStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self._type())
    }
}

pub struct RemoveExtraCategories {}

impl ExtraCategoriesStrategy for RemoveExtraCategories {
    fn _type(&self) -> ExtraCategoriesStrategyType {
        ExtraCategoriesStrategyType::Remove
    }

    fn handle_extra_category(
        &self,
        extra_category: &ExistingCategory,
        commands: &mut Vec<CommandRef>,
    ) {
        let command = DeleteCategory::new(extra_category.clone());
        commands.push(Arc::from(command));
    }
}

pub struct KeepExtraCategories {}

impl ExtraCategoriesStrategy for KeepExtraCategories {
    fn _type(&self) -> ExtraCategoriesStrategyType {
        ExtraCategoriesStrategyType::Keep
    }

    fn handle_extra_category(
        &self,
        _extra_category: &ExistingCategory,
        _commands: &mut Vec<CommandRef>,
    ) {
    }
}
