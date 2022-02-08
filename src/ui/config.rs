use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::api::files::Files;

pub struct ConfigApi {
    started_daily: bool,
    file_config: Files,
    devs_turn: Vec::<String>
}

impl ConfigApi {

    pub fn new() -> Self {
        let file_config = Files::new();
        ConfigApi {
            started_daily: false,
            file_config,
            devs_turn: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        // randomize the turn
        let mut devs = self.file_config.get_devs().to_vec();
        devs.shuffle(&mut thread_rng());
        self.devs_turn = devs;
        self.started_daily = true;
    }

    pub fn is_started(&self) -> &bool  {
        return &self.started_daily;
    }

    pub fn get_devs(&self) -> &Vec::<String> {
        return &self.file_config.get_devs();
    }

    pub fn add_dev(&mut self, name: &str) {
        if *self.is_started() {
            panic!("Can't modify devs when daily has started");
        }
        self.file_config.add_dev(name);
    }

    pub fn delete_dev(&mut self, index: usize) {
        if *self.is_started() {
            panic!("Can't modify devs when daily has started");
        }
        self.file_config.delete_dev(index);
    }

    pub fn get_turns(&self) -> &Vec::<String> {
        return &self.devs_turn;
    }

    pub fn next(&mut self) {
        self.devs_turn.remove(0);
    }

    pub fn skip(&mut self) {
        let current_dev_turn = self.devs_turn.remove(0);
        self.devs_turn.push(current_dev_turn);
    }
}

impl Default for ConfigApi {
    fn default() -> Self {
        Self::new()
    }
}