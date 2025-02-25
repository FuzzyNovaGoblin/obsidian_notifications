use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Vault {
    pub destination: String,
    pub debug_destination: String,
    pub root_dir: String,
    #[serde(default = "default_todo_path")]
    pub todo_path: String,
    #[serde(default = "default_todo_header")]
    pub todo_section_header: String,
}
fn default_todo_path() -> String {
    String::from("General/TODO.md")
}
fn default_todo_header() -> String {
    String::from("TODO")
}
