use std::fs;
use std::sync::Arc;
use bevy::prelude::{Plugin, Resource, App};
use serde::Deserialize;


pub struct ConfigPlugin{
    loaded_config: Result<LoadedConf, std::io::Error>
}

impl ConfigPlugin{
    pub fn new(loaded_config: Result<LoadedConf, std::io::Error>)->Self{
        Self{loaded_config}
    }
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let used_config = match self.loaded_config{
            Ok(ref loaded_config)=> loaded_config.clone(),
            Err(ref error )=>{
                bevy::log::warn!("Failed to load config: {}", error);
                LoadedConf::default()
            }
        };

        bevy::log::debug!("Config loaded: \n{:?}", used_config);

        app.insert_resource(LoadedConf::default());
    }
}
#[derive(Resource, Default, Debug, Clone)]
pub struct LoadedConf{
    configuration: Arc<Configuration>,
}

impl LoadedConf{
    pub fn new(configuration: Configuration)->Self{
        Self{configuration: Arc::new(configuration)}
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