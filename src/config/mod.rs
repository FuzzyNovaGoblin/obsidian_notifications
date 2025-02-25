use self::destination::Destination;
use crate::bot::context::DisCtx;
use std::{collections::HashMap, sync::Arc};
use shared_singleton::*;

pub mod destination;
pub mod vault;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub bot_token: String,
    pub rust_error_ch: String,
    pub destinations: HashMap<String, destination::Destination>,
    pub vaults: HashMap<String, vault::Vault>,
}

impl_singleton_arc!(Config, Config::load());

impl Config {
    pub fn load() -> Config {
        let data = std::fs::read_to_string("/etc/obsidian_notifications/config.toml")
            .expect("failed to read config at '/etc/obsidian_notifications/config.toml'");

        match toml::from_str(&data) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "error in config file '/etc/obsidian_notifications/config.toml'\n{}",
                    e
                );
                panic!()
            }
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub dis_ctx: DisCtx,
    pub config: Config,
}

impl Context {
    // pub fn get_vault_name_from_dest(&self, dest: &Destination) -> &str{
    //     self.config.vaults.iter().filter_map(|(name, vlt)|if vlt. == dest.)
    // }

    pub fn get_error_ch(&self) -> &Destination {
        &self.config.destinations[&self.config.rust_error_ch]
    }

    pub fn get_dest(&self, dest_name: impl AsRef<str>)->&Destination{
        &self.config.destinations[dest_name.as_ref()]
    }

    pub fn get_vault_dest(&self, vault_name: impl AsRef<str>)->&Destination{
        &self.config.destinations[&*self.config.vaults[vault_name.as_ref()].destination]
    }

    pub fn get_vault_dbg_dest(&self, vault_name: impl AsRef<str>)->&Destination{
        &self.config.destinations[&*self.config.vaults[vault_name.as_ref()].debug_destination]
    }

    pub fn get_all_vault_names(&self) -> impl Iterator< Item = Arc<String>> + '_ {
        self.config.vaults.keys().map(|k| Arc::new(k.clone()))
    }
}
