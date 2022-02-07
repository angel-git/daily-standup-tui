use std::fs;
use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;

const CONFIG_FILE: &str = "config.toml";
const APP_DIR: &str = ".daily-standup-tui";

pub struct Files {
    config_file: PathBuf,
    config_app: FileConfig,
}


#[derive(Deserialize, Serialize)]
struct FileConfig {
    devs: DevelopersConfig,
    settings: Option<SettingsConfig>,
}

#[derive(Deserialize, Serialize)]
struct DevelopersConfig {
    names: Vec::<String>,
}

#[derive(Deserialize, Serialize)]
struct SettingsConfig {
    seconds: u16,
}


impl Files {
    pub fn new() -> Files {
        let mut files = Files {
            config_file: PathBuf::new(),
            config_app: FileConfig {
                devs: DevelopersConfig {
                    names: Vec::from(["Example Dev 1", "Example Dev 2"]).into_iter().map(String::from).collect()
                },
                settings: Some(SettingsConfig {
                    seconds: 180
                }),
            }
        };

        files.load_file_config();
        files
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
        self.config_file = config_file.clone();

        if !config_file.exists() {
            self.write_toml_file();
        } else {
            let toml_string = fs::read_to_string(config_file).unwrap();
            match toml::from_str::<FileConfig>(toml_string.as_str()) {
                Ok(conf) => {
                    self.config_app.settings = conf.settings;
                    self.config_app.devs = conf.devs;
                }
                Err(e) => panic!("Error while parsing config file {}", e)
            }
        }
    }

    fn write_toml_file(&self) {
        let toml_string = toml::to_string(&self.config_app).expect("Can't parse toml default values");
        match fs::write(self.config_file.clone(), toml_string) {
            Ok(_) => (),
            Err(e) => panic!("Error while writing config file {}", e)
        }
    }

    pub fn get_devs(&self) -> &Vec<String> {
        return &self.config_app.devs.names;
    }

    pub fn add_dev(&mut self, name: &str) {
        let names = &mut self.config_app.devs.names;
        names.push(name.to_string());
        self.write_toml_file();
    }

    pub fn delete_dev(&mut self, index: usize) {
        let names = &mut self.config_app.devs.names;
        names.remove(index);
        self.write_toml_file();
    }
}


impl Default for Files {
    fn default() -> Self {
        Self::new()
    }
}