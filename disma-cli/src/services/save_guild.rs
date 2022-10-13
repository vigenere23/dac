use colored::Colorize;
use std::{path::Path, sync::Arc};

use crate::{
    infra::config::guild::GuildConfig,
    utils::{
        input::{abort, ask_user_confirmation},
        io::Serializer,
    },
};
use disma::guild::GuildQuerier;

pub struct SaveExistingGuild {
    guild_querier: Arc<dyn GuildQuerier>,
    serializer: Arc<Serializer>,
}

impl SaveExistingGuild {
    pub fn new(guild_querier: Arc<dyn GuildQuerier>, serializer: Arc<Serializer>) -> Self {
        Self {
            guild_querier,
            serializer,
        }
    }

    pub fn run(&self, guild_id: &str, file: &str, force: bool) {
        let guild = self.guild_querier.get_guild(guild_id);

        let config = GuildConfig::from(&guild);
        let file_path = Path::new(file);

        println!();
        println!(
            "{}",
            format!("🡲 💾 Saving current guild config to '{}'...", file).bold()
        );

        if !force && file_path.exists() {
            println!(
                "{}",
                format!("🡲 ❗ A file named '{}' already exists.", file).bold()
            );

            if !ask_user_confirmation("Do you still want to proceeed?") {
                abort();
            }
        }

        self.serializer.serialize(&config, file_path);

        println!("{}", "🡲 ✨ DONE.".bold());
    }
}
