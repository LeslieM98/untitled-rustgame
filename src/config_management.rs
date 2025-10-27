use std::fs;
use bevy::prelude::{Plugin, Resource, App};
use serde::Deserialize;


pub struct ConfigPlugin;
type FileName = String;


#[derive(Resource)]
pub struct ConfigDir{
    directory: String,
    file_ending: String,
}

impl Default for ConfigDir {
    fn default() -> Self {
        Self{directory: String::from("config"), file_ending: String::from("yaml") }
    }
}

#[derive(Resource, Deserialize, Default, Debug)]
pub struct Configuration{
    general: General,
    debug: Option<Debug>
}

#[derive(Deserialize, Default, Debug)]
pub struct General {
    extra_args: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Debug {
    enable_logging: bool
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConfigDir>();
        app.insert_resource(load_config(app.world().resource::<ConfigDir>()).unwrap_or_else(|e| {
            let default = Configuration::default();
            bevy::log::warn!("Failed to load config: {}", e);
            bevy::log::warn!("Default config loaded!");
            bevy::log::debug!("Config content:\n{:?}", default);
            default
        }));
    }
}

fn load_config(config_dir: &ConfigDir) -> Result<Configuration, std::io::Error>{
    let paths = fs::read_dir(&config_dir.directory)?;
    let mut file_contents: Vec<String> = vec![];
    for path in paths {
        let file_content = fs::read_to_string(path?.path())?;
        file_contents.push(file_content);
    }

    let content_concat: String = file_contents.join("\n");

    match toml::from_str(&content_concat) {
        Ok(config) => {
            bevy::log::info!("Config loaded successfully");
            bevy::log::debug!("Config content:\n{:?}", config);
            Ok(config)
        },
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}