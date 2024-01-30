use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub add_repository_command: String,
    pub repositories: Vec<String>,
    pub sync_command: String,
    pub update_command: String,
    pub install_command: String,
    pub packages: Vec<String>,
    pub grouped_packages: Vec<PackageGroup>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self { 
            add_repository_command: String::new(),
            repositories: Vec::new(), 
            sync_command: String::new(),
            update_command: String::new(),
            install_command: String::new(),
            packages: Vec::new(),
            grouped_packages: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageGroup {
    pub group_name: String,
    pub packages: Vec<String>,
}
