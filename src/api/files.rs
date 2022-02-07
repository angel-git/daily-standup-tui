use std::fs;
use std::fs::File;
use std::io::Write;
use serde::Deserialize;
use serde::Serialize;

const CONFIG_FILE: &str = "config.toml";
const APP_DIR: &str = ".daily-standup-tui";

#[derive(Deserialize, Serialize)]
pub struct FileConfig {
    devs: DevelopersConfig,
    settings: Option<SettingsConfig>
}

#[derive(Deserialize, Serialize)]
struct DevelopersConfig {
    names: Vec::<String>,
}

#[derive(Deserialize, Serialize)]
struct SettingsConfig {
    seconds: u16,
}


impl FileConfig {

    pub fn new() -> FileConfig {

        let mut file_config = FileConfig {
            devs: DevelopersConfig {
                names: Vec::from(["Example Dev 1", "Example Dev 2"]).into_iter().map(String::from).collect()
            },
            settings: Some(SettingsConfig {
                seconds: 180
            }),
        };

        file_config.load_file_config();
        file_config
    }

    fn load_file_config(&mut self) {
        let app_dir_in_home_folder = match dirs::home_dir() {
            Some(dir) => {
                dir.join(APP_DIR)
            }
            None => panic!("User home folder not found")
        };


        if !app_dir_in_home_folder.exists() {
            match fs::create_dir(&app_dir_in_home_folder) {
                Ok(_) => (),
                Err(e) => panic!("Error creating config file into home folder: {}", e)
            }
        }

        let config_file = app_dir_in_home_folder.join(CONFIG_FILE);

        if !config_file.exists() {
            // let file = File::create(config_file);
            let toml_string = toml::to_string(self).expect("Can't parse toml default values");
            fs::write(config_file, toml_string);
        } else {
            let toml_string = fs::read_to_string(config_file).unwrap();
            match toml::from_str::<FileConfig>(toml_string.as_str())  {
                Ok(conf) => {
                    self.settings = conf.settings;
                    self.devs = conf.devs;
                },
                Err(e) => panic!("Error while parsing config file {}", e)
            }
        }
    }

    pub fn get_devs(&self) -> &Vec<String> {
        return &self.devs.names;
    }



}


impl Default for FileConfig {
    fn default() -> Self {
        Self::new()
    }
}