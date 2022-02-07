use crate::api::files::Files;

pub struct ConfigApi {
    started_daily: bool,
    file_config: Files,
}

impl ConfigApi {

    pub fn new() -> Self {
        let file_config = Files::new();
        ConfigApi {
            started_daily: false,
            file_config,
        }
    }

    pub fn start(&mut self) {
        self.started_daily = true;
    }

    pub fn is_started(&self) -> &bool  {
        return &self.started_daily;
    }

    pub fn get_devs(&self) -> &Vec::<String> {
        return &self.file_config.get_devs();
    }

    pub fn add_dev(&mut self, name: &str) {
        self.file_config.add_dev(name);
    }

    pub fn delete_dev(&mut self, index: usize) {
        self.file_config.delete_dev(index);
    }
}

impl Default for ConfigApi {
    fn default() -> Self {
        Self::new()
    }
}