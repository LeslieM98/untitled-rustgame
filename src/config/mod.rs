use std::fs;
use std::sync::Arc;
use bevy::prelude::{Plugin, Resource, App};
use serde::Deserialize;


pub struct ConfigPlugin{
    config: LoadedConf
}

impl ConfigPlugin{
    pub fn new(loaded_config: LoadedConf)->Self{
        Self{ config: loaded_config }
    }
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        bevy::log::debug!("Config loaded: \n{:?}", self.config);

        app.insert_resource(LoadedConf::default());
    }
}
#[derive(Resource, Default, Debug)]
pub struct LoadedConf{
    pub configuration: Configuration
}

impl LoadedConf{
    pub fn new(configuration: Configuration)->Self{
        Self{configuration }
    }
}

#[derive(Resource, Deserialize, Default, Debug)]
pub struct Configuration{
    pub general: General,
    pub debug: Option<Debug>
}

#[derive(Deserialize, Default, Debug)]
pub struct General {
    pub extra_args: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Debug {
    pub enable_debug_logging: bool
}

pub fn load_config(config_dir: &str) -> Result<LoadedConf, std::io::Error> {
    let paths = fs::read_dir(config_dir)?;
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
            Ok(LoadedConf::new(config))
        },
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}