use crate::bot::destination::Destination;

pub const FUZ_VAULT_PATH: &str = "/home/fuzzy/obsidian/fuz-vault";
pub const DEBUG_PATH: &str = "/home/fuzzy/obsidian/fuz-vault/debug";
pub const MAGIC_BEANS_VAULT_PATH: &str = "/home/fuzzy/obsidian/magic-beans-vault";


#[derive(Clone)]
pub struct FileSearchConfig {
    pub root_dir: &'static str,
    pub notification_channel: Destination,
}


impl FileSearchConfig {
    pub fn gen_all_configs() -> Vec<FileSearchConfig> {
        vec![
            FileSearchConfig {
                root_dir: FUZ_VAULT_PATH,
                notification_channel: Destination::FuzzyObsidianCh,
            },
            FileSearchConfig {
                root_dir: MAGIC_BEANS_VAULT_PATH,
                notification_channel: Destination::MagicBeansObsidianCh,
            },
        ]
    }

    pub fn gen_all_debug_configs() -> Vec<FileSearchConfig> {
        vec![
            FileSearchConfig {
                root_dir: FUZ_VAULT_PATH,
                notification_channel: Destination::DebugObsidianFuzzyCh,
            },
            FileSearchConfig {
                root_dir: MAGIC_BEANS_VAULT_PATH,
                notification_channel: Destination::DebugObsidianMagicBeansCh,
            },
        ]
    }

    pub fn gen_short_debug_config() -> Vec<FileSearchConfig>{
        vec![
            FileSearchConfig {
                root_dir: DEBUG_PATH,
                notification_channel: Destination::DebugObsidianFuzzyCh,
            },
        ]
    }
}
