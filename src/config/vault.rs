use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Vault {
    pub destination: String,
    pub debug_destination: String,
    pub root_dir: String,
}
