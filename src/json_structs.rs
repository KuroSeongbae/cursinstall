use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonConfig {
    pub title: String,
    pub comment: String,
    pub commands: Vec<JsonCmd>
}

#[derive(Deserialize, Debug)]
pub struct JsonCmd {
    pub command: String,
    pub args: Vec<String>,
    pub grouped_args: Vec<JsonArgGroup>
}

#[derive(Deserialize, Debug)]
pub struct JsonArgGroup {
    pub group_name: String,
    pub args: Vec<String>,
}
