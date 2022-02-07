use std::borrow::Borrow;
use crate::api::files::FileConfig;

pub struct ConfigApi {
    started_daily: bool,
    devs: Vec::<String>,
}

impl ConfigApi {

    pub fn new() -> Self {
        let file_config = FileConfig::new();
        ConfigApi {
            started_daily: false,
            devs: file_config.get_devs().to_vec(),
        }
    }

    pub fn start(&mut self) {
        self.started_daily = true;
    }

    pub fn is_started(&self) -> &bool  {
        return &self.started_daily;
    }

    pub fn get_devs(&self) -> &Vec::<String> {
        return &self.devs;
    }
}

impl Default for ConfigApi {
    fn default() -> Self {
        Self::new()
    }
}